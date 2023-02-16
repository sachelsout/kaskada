//! Basic e2e tests for the comparison operators.
// gt, lt, gte, lte: i64, f64, timestamp, literal

use crate::fixtures::{f64_data_fixture, i64_data_fixture, timestamp_ns_data_fixture};
use crate::QueryFixture;

#[tokio::test]
async fn test_lt_i64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, n: Numbers.n, lt: Numbers.m < Numbers.n }").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,10,true
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,3,false
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,6,false
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,9,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,,
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,,
    "###);
}

#[tokio::test]
async fn test_lt_f64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, n: Numbers.n, lt: Numbers.m < Numbers.n}").run_to_csv(&f64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5.2,10.0,true
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24.3,3.9,false
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17.6,6.2,false
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,9.25,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12.4,,
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,,
    "###);
}

#[tokio::test]
async fn test_lt_timestamp_ns() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Times.m, n: Times.n, lt: (Times.m as timestamp_ns) < (Times.n as timestamp_ns) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,lt
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,4,2,false
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,3,4,true
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,,5,
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,,,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,8,8,false
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,11,23,true
    "###);
}

#[tokio::test]
async fn test_lt_i64_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lt: Numbers.m < 10}").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,true
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,false
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,false
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,false
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,
    "###);
}

#[tokio::test]
async fn test_lt_f64_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lt: Numbers.m < 10.0}").run_to_csv(&f64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5.2,true
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24.3,false
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17.6,false
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12.4,false
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,
    "###);
}

#[tokio::test]
async fn test_lt_timestamp_ns_literal() {
    insta::assert_snapshot!(QueryFixture::new("
        let m_time = Times.m as timestamp_ns
        let literal = \"1970-01-01T00:00:00.000000010Z\"
        in { m_time, literal_time: literal as timestamp_ns, lt: m_time < literal}").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m_time,literal_time,lt
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1970-01-01T00:00:00.000000004,1970-01-01T00:00:00.000000010,true
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000003,1970-01-01T00:00:00.000000010,true
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,,1970-01-01T00:00:00.000000010,
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,,1970-01-01T00:00:00.000000010,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000008,1970-01-01T00:00:00.000000010,true
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000011,1970-01-01T00:00:00.000000010,false
    "###);
}

#[tokio::test]
async fn test_gt_i64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, n: Numbers.n, gt: Numbers.m > Numbers.n }").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,gt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,10,false
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,3,true
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,6,true
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,9,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,,
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,,
    "###);
}

#[tokio::test]
async fn test_gt_f64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, n: Numbers.n, gt: Numbers.m > Numbers.n}").run_to_csv(&f64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,gt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5.2,10.0,false
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24.3,3.9,true
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17.6,6.2,true
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,9.25,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12.4,,
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,,
    "###);
}

#[tokio::test]
async fn test_gt_timestamp_ns() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Times.m, n: Times.n, gt: (Times.m as timestamp_ns) > (Times.n as timestamp_ns) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,gt
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,4,2,true
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,3,4,false
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,,5,
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,,,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,8,8,false
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,11,23,false
    "###);
}

#[tokio::test]
async fn test_gt_i64_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lt: Numbers.m > 10}").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,false
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,true
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,true
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,true
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,
    "###);
}

#[tokio::test]
async fn test_gt_f64_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lt: Numbers.m > 10.0}").run_to_csv(&f64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5.2,false
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24.3,true
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17.6,true
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12.4,true
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,
    "###);
}

#[tokio::test]
async fn test_gt_timestamp_ns_literal() {
    insta::assert_snapshot!(QueryFixture::new("
        let m_time = Times.m as timestamp_ns
        let literal = \"1970-01-01T00:00:00.000000010Z\"
        in { m_time, literal_time: literal as timestamp_ns, gt: m_time > literal}").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m_time,literal_time,gt
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1970-01-01T00:00:00.000000004,1970-01-01T00:00:00.000000010,false
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000003,1970-01-01T00:00:00.000000010,false
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,,1970-01-01T00:00:00.000000010,
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,,1970-01-01T00:00:00.000000010,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000008,1970-01-01T00:00:00.000000010,false
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000011,1970-01-01T00:00:00.000000010,true
    "###);
}

#[tokio::test]
async fn test_lte_i64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, n: Numbers.n, lte: Numbers.m <= Numbers.n }").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,lte
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,10,true
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,3,false
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,6,false
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,9,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,,
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,,
    "###);
}

#[tokio::test]
async fn test_lte_f64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, n: Numbers.n, lte: Numbers.m <= Numbers.n}").run_to_csv(&f64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,lte
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5.2,10.0,true
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24.3,3.9,false
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17.6,6.2,false
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,9.25,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12.4,,
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,,
    "###);
}

#[tokio::test]
async fn test_lte_timestamp_ns() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Times.m, n: Times.n, lte: (Times.m as timestamp_ns) <= (Times.n as timestamp_ns) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,lte
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,4,2,false
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,3,4,true
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,,5,
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,,,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,8,8,true
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,11,23,true
    "###);
}

#[tokio::test]
async fn test_lte_i64_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lt: Numbers.m <= 10}").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,true
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,false
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,false
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,false
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,
    "###);
}

#[tokio::test]
async fn test_lte_f64_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lt: Numbers.m <= 10.0}").run_to_csv(&f64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5.2,true
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24.3,false
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17.6,false
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12.4,false
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,
    "###);
}

#[tokio::test]
async fn test_lte_timestamp_ns_literal() {
    insta::assert_snapshot!(QueryFixture::new("
        let m_time = Times.m as timestamp_ns
        let literal = \"1970-01-01T00:00:00.000000008Z\"
        in { m_time, literal_time: literal as timestamp_ns, lte: m_time <= literal}").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m_time,literal_time,lte
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1970-01-01T00:00:00.000000004,1970-01-01T00:00:00.000000008,true
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000003,1970-01-01T00:00:00.000000008,true
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,,1970-01-01T00:00:00.000000008,
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,,1970-01-01T00:00:00.000000008,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000008,1970-01-01T00:00:00.000000008,true
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000011,1970-01-01T00:00:00.000000008,false
    "###);
}

#[tokio::test]
async fn test_gte_i64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, n: Numbers.n, gte: Numbers.m >= Numbers.n }").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,gte
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,10,false
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,3,true
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,6,true
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,9,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,,
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,,
    "###);
}

#[tokio::test]
async fn test_gte_f64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, n: Numbers.n, gte: Numbers.m >= Numbers.n}").run_to_csv(&f64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,gte
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5.2,10.0,false
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24.3,3.9,true
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17.6,6.2,true
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,9.25,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12.4,,
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,,
    "###);
}

#[tokio::test]
async fn test_gte_timestamp_ns() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Times.m, n: Times.n, gte: Times.m >= Times.n}").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,n,gte
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,4,2,true
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,3,4,false
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,,5,
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,,,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,8,8,true
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,11,23,false
    "###);
}

#[tokio::test]
async fn test_gte_i64_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lt: Numbers.m >= 10}").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,false
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,true
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,true
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,true
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,
    "###);
}

#[tokio::test]
async fn test_gte_f64_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lt: Numbers.m >= 10.0}").run_to_csv(&f64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lt
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5.2,false
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24.3,true
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17.6,true
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12.4,true
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,
    "###);
}

#[tokio::test]
async fn test_gte_timestamp_ns_literal() {
    insta::assert_snapshot!(QueryFixture::new("
        let m_time = Times.m as timestamp_ns
        let literal = \"1970-01-01T00:00:00.000000008Z\"
        in { m_time, literal_time: literal as timestamp_ns, gte: m_time >= literal}").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m_time,literal_time,gte
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1970-01-01T00:00:00.000000004,1970-01-01T00:00:00.000000008,false
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000003,1970-01-01T00:00:00.000000008,false
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,,1970-01-01T00:00:00.000000008,
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,,1970-01-01T00:00:00.000000008,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000008,1970-01-01T00:00:00.000000008,true
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1970-01-01T00:00:00.000000011,1970-01-01T00:00:00.000000008,true
    "###);
}
