use serde_derive::Deserialize;
use std::borrow::Cow;

// A line entry in the log files
#[derive(Deserialize, Debug)]
pub struct Entry<'a> {
    // "2019-08-28T01:59:59+0000"
    #[serde(borrow)]
    pub timestamp: Cow<'a, str>,

    // "2607:fb90:533d:e9eb:b0ef:bcdb:599b:faa8"
    #[serde(borrow)]
    pub client_ip: Cow<'a, str>,

    // GET
    pub method: HttpMethod,

    // "HTTP/1.1",
    // protocol: Cow<'a, str>,

    // "2607:fb90:533d:e9eb:b0ef:bcdb:599b:faa8"
    // host: Cow<'a, str>,

    // "www.wearecms.com"
    #[serde(borrow)]
    pub origin_host: Cow<'a, str>,

    // "/pics/search_button.png"
    #[serde(borrow)]
    pub url_path: Cow<'a, str>,

    #[serde(borrow)]
    pub url_query_string: Cow<'a, str>,

    // "304",
    #[serde(borrow)]
    pub status: Cow<'a, str>,

    // https://www.wearecms.com/apps/pages/cmsstudentportal
    #[serde(borrow)]
    pub request_referrer: Option<Cow<'a, str>>,

    // "Mozilla/5.0 (iPhone; CPU iPhone OS 12_4 like Mac OS X).."
    #[serde(borrow)]
    pub request_user_agent: Cow<'a, str>,

    // bytes_received: usize,                // 832
    // bytes_sent_excl_headers: usize,       // 0
    // bytes_sent_incl_headers: usize,       // 0
    // microseconds_serve_request: usize,    // 532

    // "HIT-STALE-CLUSTER:
    pub cache_status: CacheStatus,
}

#[serde(rename_all = "UPPERCASE")]
#[derive(Deserialize, Debug)]
pub enum HttpMethod {
    Head,
    Options,
    Get,
    GGet,
    GeGet,
    Post,
    Put,
    Patch,
    Delete,
    PropFind,
    #[serde(rename = "CCM_POST")]
    CCMPost,
}

#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
#[derive(Deserialize, Debug)]
pub enum CacheStatus {
    Hit,
    HitWait,
    HitWaitCluster,
    Pass,
    PassWait,
    Miss,
    MissWait,
    #[serde(rename = "HITPASS")]
    HitPass,
    #[serde(rename = "HITPASS-WAIT")]
    HitPassWait,
    HitStale,
    HitStaleWait,
    HitStaleWaitCluster,
    Error,
    ErrorWait,
    MissCluster,
    MissWaitCluster,
    HitCluster,
    HitStaleCluster,
}
