use arrow::array::{StringArray, TimestampNanosecondArray, UInt64Array};
use indoc::indoc;
use sparrow_api::kaskada::v1alpha::TableConfig;
use uuid::Uuid;

use crate::{DataFixture, QueryFixture};

#[tokio::test]
async fn test_prepare_default_subsort_parquet() {
    let table = indoc! {"
    time,key,m,n
    1996-12-19T16:39:57-08:00,A,5,10
    1996-12-19T16:39:58-08:00,B,24,3
    1996-12-19T16:39:59-08:00,A,17,6
    1996-12-19T16:40:00-08:00,A,,9
    1996-12-19T16:40:01-08:00,A,12,
    1996-12-19T16:40:02-08:00,A,,
    "};

    let data_fixture = DataFixture::new()
        .with_table_from_csv(
            TableConfig::new_with_table_source("Numbers", &Uuid::new_v4(), "time", None, "key", ""),
            table,
        )
        .await
        .unwrap();

    insta::assert_snapshot!(QueryFixture::new("Numbers").run_to_csv(&data_fixture).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,key,m,n
    1996-12-20T00:39:57.000000000,14798209437330752939,12960666915911099378,A,1996-12-20T00:39:57.000000000,A,5,10
    1996-12-20T00:39:58.000000000,14798209437330752940,2867199309159137213,B,1996-12-20T00:39:58.000000000,B,24,3
    1996-12-20T00:39:59.000000000,14798209437330752941,12960666915911099378,A,1996-12-20T00:39:59.000000000,A,17,6
    1996-12-20T00:40:00.000000000,14798209437330752942,12960666915911099378,A,1996-12-20T00:40:00.000000000,A,,9
    1996-12-20T00:40:01.000000000,14798209437330752943,12960666915911099378,A,1996-12-20T00:40:01.000000000,A,12,
    1996-12-20T00:40:02.000000000,14798209437330752944,12960666915911099378,A,1996-12-20T00:40:02.000000000,A,,
    "###);
}

#[tokio::test]
async fn test_prepare_key_columns_parquet() {
    let table = indoc! {"
    time,subsort,key,m,n
    1996-12-19T16:39:57-08:00,1,A,5,10
    1996-12-19T16:39:58-08:00,2,B,24,3
    1996-12-19T16:39:59-08:00,3,A,17,6
    1996-12-19T16:40:00-08:00,4,A,,9
    1996-12-19T16:40:01-08:00,5,A,12,
    1996-12-19T16:40:02-08:00,6,A,,
    "};

    let data_fixture = DataFixture::new()
        .with_table_from_csv(
            TableConfig::new_with_table_source(
                "Numbers",
                &Uuid::new_v4(),
                "time",
                Some("subsort"),
                "key",
                "",
            ),
            table,
        )
        .await
        .unwrap();
    insta::assert_snapshot!(QueryFixture::new("Numbers").run_to_csv(&data_fixture).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,subsort,key,m,n
    1996-12-20T00:39:57.000000000,9223372036854775809,12960666915911099378,A,1996-12-20T00:39:57.000000000,1,A,5,10
    1996-12-20T00:39:58.000000000,9223372036854775810,2867199309159137213,B,1996-12-20T00:39:58.000000000,2,B,24,3
    1996-12-20T00:39:59.000000000,9223372036854775811,12960666915911099378,A,1996-12-20T00:39:59.000000000,3,A,17,6
    1996-12-20T00:40:00.000000000,9223372036854775812,12960666915911099378,A,1996-12-20T00:40:00.000000000,4,A,,9
    1996-12-20T00:40:01.000000000,9223372036854775813,12960666915911099378,A,1996-12-20T00:40:01.000000000,5,A,12,
    1996-12-20T00:40:02.000000000,9223372036854775814,12960666915911099378,A,1996-12-20T00:40:02.000000000,6,A,,
    "###);
}

#[tokio::test]
async fn test_u64_key() {
    let table = crate::ParquetTableBuilder::new()
        .add_column(
            "time",
            false,
            TimestampNanosecondArray::from(vec![1000, 1001, 1002, 1003]),
        )
        .add_column(
            "not_a_key",
            true,
            StringArray::from(vec![Some("r0"), Some("r1"), None, Some("r4")]),
        )
        .add_column(
            "user_id",
            true,
            UInt64Array::from(vec![Some(0), Some(1), Some(2), Some(4)]),
        );

    let data_fixture = DataFixture::new()
        .with_table_from_parquet(
            TableConfig::new_with_table_source(
                "Events",
                &Uuid::new_v4(),
                "time",
                None,
                "user_id",
                "user",
            ),
            table,
        )
        .await
        .unwrap();
    insta::assert_snapshot!(QueryFixture::new("Events").run_to_csv(&data_fixture).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,not_a_key,user_id
    1970-01-01T00:00:00.000001000,17227329910109684520,11832085162654999889,0,1970-01-01T00:00:00.000001000,r0,0
    1970-01-01T00:00:00.000001001,17227329910109684521,18433805721903975440,1,1970-01-01T00:00:00.000001001,r1,1
    1970-01-01T00:00:00.000001002,17227329910109684522,2694864431690786590,2,1970-01-01T00:00:00.000001002,,2
    1970-01-01T00:00:00.000001003,17227329910109684523,17062639839782733832,4,1970-01-01T00:00:00.000001003,r4,4
    "###);
}
