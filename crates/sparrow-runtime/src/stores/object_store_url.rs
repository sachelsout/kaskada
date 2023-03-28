use std::{path::PathBuf, str::FromStr};

use error_stack::{IntoReport, ResultExt};
use tokio_util::io::StreamReader;
use url::Url;

use itertools::Itertools;

use crate::ObjectStoreRegistry;

use super::object_stores::Error;

/// A string referring to a file in an object store.
/// TODO: Debug this. It doesn't like the serialize on the URL.
// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
// #[serde(transparent)]
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ObjectStoreUrl(Url);

impl ObjectStoreUrl {
    pub fn path(&self) -> error_stack::Result<object_store::path::Path, Error> {
        object_store::path::Path::parse(self.0.path())
            .into_report()
            .change_context_lazy(|| Error::UrlInvalidPath(self.0.clone()))
    }

    pub fn key(&self) -> error_stack::Result<ObjectStoreKey, Error> {
        match self.0.scheme() {
            "file" => Ok(ObjectStoreKey::Local),
            "mem" => Ok(ObjectStoreKey::Memory),
            // S3 is the traditional S3 prefix for reading from S3.
            // S3a is the protocol designed for scalability with Hadoop reading in mind.
            // See: https://aws.amazon.com/blogs/opensource/community-collaboration-the-s3a-story/
            "s3" | "s3a" => {
                let bucket = self
                    .0
                    .host_str()
                    .ok_or_else(|| Error::UrlMissingHost(self.0.clone()))?
                    .to_owned();
                // For traditional S3 paths, the `host` should be just the bucket.
                // We use this as the key. The creation of the S3 object store will
                // parse out the bucket and other parts of the URL as needed.
                Ok(ObjectStoreKey::Aws {
                    bucket,
                    region: None,
                    virtual_hosted_style_request: false,
                })
            }
            "https" => {
                let host = self
                    .0
                    .host_str()
                    .ok_or_else(|| Error::UrlMissingHost(self.0.clone()))?;

                match host.splitn(4, '.').collect_tuple() {
                    Some(("s3", bucket, "amazonaws", "com")) => Ok(ObjectStoreKey::Aws {
                        bucket: bucket.to_owned(),
                        region: None,
                        virtual_hosted_style_request: false,
                    }),
                    Some((bucket, "s3", region, "amazonaws.com")) => Ok(ObjectStoreKey::Aws {
                        bucket: bucket.to_owned(),
                        region: Some(region.to_owned()),
                        virtual_hosted_style_request: true,
                    }),
                    Some((bucket, "storage", "googleapis", "com")) => Ok(ObjectStoreKey::Gcs {
                        bucket: bucket.to_owned(),
                    }),
                    Some(("storage", "cloud", "google", "com")) => {
                        let mut path = self
                            .0
                            .path_segments()
                            .ok_or_else(|| Error::UrlInvalidPath(self.0.clone()))?;
                        let bucket = path
                            .next()
                            .ok_or_else(|| Error::UrlInvalidPath(self.0.clone()))?;
                        Ok(ObjectStoreKey::Gcs {
                            bucket: bucket.to_owned(),
                        })
                    }
                    _ => error_stack::bail!(Error::UrlUnsupportedHost(self.0.clone())),
                }
            }
            "gs" => {
                let bucket = self
                    .0
                    .host_str()
                    .ok_or_else(|| Error::UrlMissingHost(self.0.clone()))?
                    .to_owned();
                Ok(ObjectStoreKey::Gcs { bucket })
            }
            _ => {
                error_stack::bail!(Error::UrlUnsupportedScheme(self.0.clone()))
            }
        }
    }
    pub async fn download(
        &self,
        object_store_registry: &ObjectStoreRegistry,
        file_path: PathBuf,
    ) -> error_stack::Result<(), Error> {
        let path = self.path()?;
        let object_store = object_store_registry.object_store(self.key()?)?;
        let stream = object_store
            .get(&path)
            .await
            .into_report()
            .change_context_lazy(|| Error::DownloadingObject(file_path.clone()))?
            .into_stream();
        let mut file = tokio::fs::File::create(file_path.clone())
            .await
            .into_report()
            .change_context_lazy(|| Error::DownloadingObject(file_path.clone()))?;
        let mut body = StreamReader::new(stream);
        let bytes = tokio::io::copy(&mut body, &mut file)
            .await
            .into_report()
            .change_context_lazy(|| Error::DownloadingObject(file_path.clone()))?;
        println!("Demo Log Line: Successfully downloaded: {:?} bytes to {:?} from {:?}", bytes, file_path.clone(), path.clone());
        Ok(())
    }
}

impl FromStr for ObjectStoreUrl {
    type Err = error_stack::Report<<Url as FromStr>::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::from_str(s).into_report().map(ObjectStoreUrl)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ObjectStoreKey {
    Local,
    Memory,
    Aws {
        bucket: String,
        region: Option<String>,
        virtual_hosted_style_request: bool,
    },
    Gcs {
        bucket: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_urls() {
        let url = ObjectStoreUrl::from_str("file:///foo").unwrap();
        assert_eq!(url.key().unwrap(), ObjectStoreKey::Local);
        assert_eq!(
            url.path().unwrap(),
            object_store::path::Path::parse("/foo").unwrap()
        );

        let url = ObjectStoreUrl::from_str("FILE:///foo").unwrap();
        assert_eq!(url.key().unwrap(), ObjectStoreKey::Local);
        assert_eq!(
            url.path().unwrap(),
            object_store::path::Path::parse("/foo").unwrap()
        );
    }

    #[test]
    fn test_memory_urls() {
        let url = ObjectStoreUrl::from_str("mem:///foo").unwrap();
        assert_eq!(url.key().unwrap(), ObjectStoreKey::Memory);
        assert_eq!(
            url.path().unwrap(),
            object_store::path::Path::parse("foo").unwrap()
        );

        let url = ObjectStoreUrl::from_str("mem:foo").unwrap();
        assert_eq!(url.key().unwrap(), ObjectStoreKey::Memory);
        assert_eq!(
            url.path().unwrap(),
            object_store::path::Path::parse("foo").unwrap()
        );
    }

    #[test]
    fn test_aws_urls() {
        let url = ObjectStoreUrl::from_str("s3://bucket/path").unwrap();
        assert_eq!(
            url.key().unwrap(),
            ObjectStoreKey::Aws {
                bucket: "bucket".to_owned(),
                region: None,
                virtual_hosted_style_request: false,
            }
        );
        assert_eq!(
            url.path().unwrap(),
            object_store::path::Path::parse("path").unwrap()
        );

        let url = ObjectStoreUrl::from_str("s3a://bucket/foo").unwrap();
        assert_eq!(
            url.key().unwrap(),
            ObjectStoreKey::Aws {
                bucket: "bucket".to_owned(),
                region: None,
                virtual_hosted_style_request: false,
            }
        );
        assert_eq!(
            url.path().unwrap(),
            object_store::path::Path::parse("foo").unwrap()
        );

        let url = ObjectStoreUrl::from_str("https://s3.bucket.amazonaws.com/foo").unwrap();
        assert_eq!(
            url.key().unwrap(),
            ObjectStoreKey::Aws {
                bucket: "bucket".to_owned(),
                region: None,
                virtual_hosted_style_request: false,
            }
        );
        assert_eq!(
            url.path().unwrap(),
            object_store::path::Path::parse("foo").unwrap()
        );

        let url = ObjectStoreUrl::from_str("https://bucket.s3.region.amazonaws.com/foo").unwrap();
        assert_eq!(
            url.key().unwrap(),
            ObjectStoreKey::Aws {
                bucket: "bucket".to_owned(),
                region: Some("region".to_owned()),
                virtual_hosted_style_request: true
            }
        );
        assert_eq!(
            url.path().unwrap(),
            object_store::path::Path::parse("foo").unwrap()
        );
    }

    #[test]
    fn test_gcp_urls() {
        let url = ObjectStoreUrl::from_str("gs://bucket/path").unwrap();
        assert_eq!(
            url.key().unwrap(),
            ObjectStoreKey::Gcs {
                bucket: "bucket".to_owned()
            }
        );

        let url = ObjectStoreUrl::from_str("https://bucket.storage.googleapis.com/path").unwrap();
        assert_eq!(
            url.key().unwrap(),
            ObjectStoreKey::Gcs {
                bucket: "bucket".to_owned()
            }
        );

        let url = ObjectStoreUrl::from_str("https://storage.cloud.google.com/bucket/path").unwrap();
        assert_eq!(
            url.key().unwrap(),
            ObjectStoreKey::Gcs {
                bucket: "bucket".to_owned()
            }
        );
    }
}
