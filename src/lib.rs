use url::*;

/// Contains directives on how to extract the link from a click-tracking link forwarder.
pub struct CleanInformation<'a> {
    /// The domain which is used to forward
    domain: &'a str,
    /// The path at the given domain that will the tracking-url will send tracking information to
    path: &'a str,
    /// The query parameter that the actual link of interest is sent as
    querykey: &'a str,
}

/// When these keys are part of the url query parameters, they will be removed from the link
/// So that if the parameters contain something like "www.example.com/foo?param1=bar&fbclid=1234",
/// the resulting query string will become something simlar to "www.example.com/foo?param1=bar"
/// with the click id query parameter "fbclid" removed
const KEYS_TO_CLEAN: [&'static str; 3] = ["fbclid", "custlinkid", "gclid"];

/// Five commonly used tracking forwarders that are going to be cleaned
const DOMAINS_TO_CLEAN: [CleanInformation<'static>; 5] = {
    [
        CleanInformation {
            domain: "l.facebook.com",
            path: "/l.php",
            querykey: "u",
        },
        CleanInformation {
            domain: "l.messenger.com",
            path: "/l.php",
            querykey: "u",
        },
        CleanInformation {
            domain: "www.google.com",
            path: "/url",
            querykey: "url",
        },
        CleanInformation {
            domain: "www.google.com",
            path: "/url",
            querykey: "q",
        },
        CleanInformation {
            domain: "external.fbma2-1.fna.fbcdn.net",
            path: "/safe_image.php",
            querykey: "url",
        },
    ]
};

pub struct UrlCleaner<'a> {
    /// Information on how to obtain the link from a tracking link
    cleaning_info: Vec<CleanInformation<'a>>,

    /// list of known tracking query keys
    tracker_query_keys: Vec<String>,
}

impl<'a> Default for UrlCleaner<'a> {
    fn default() -> Self {
        let cleaning_info = DOMAINS_TO_CLEAN.into();
        let tracker_query_keys = KEYS_TO_CLEAN.iter().map(|s| s.to_string()).collect();

        Self {
            cleaning_info,
            tracker_query_keys,
        }
    }
}

impl<'a> UrlCleaner<'a> {
    // remove the click-id and similar query that can sometimes come hidden inside links
    fn clean_query(&self, url: &url::Url) -> (url::Url, bool) {
        let pairs = url.query_pairs();
        let mut newurl = url.clone();
        newurl.query_pairs_mut().clear();
        let mut modified = false;

        for (key, value) in pairs {
            if self.tracker_query_keys.contains(&key.as_ref().to_string()) {
                println!("key found: {:?}", key);
                modified = true;
            } else {
                newurl.query_pairs_mut().append_pair(&key, &value);
            }
        }
        (newurl, modified)
    }

    /// try to extract the destination url from the link if possible and also try to remove the click-id
    /// query parameters that are available, if the content has been modified return Some, or if
    /// the content is untouched, return None
    pub fn clean_url(&self, url: &url::Url) -> Option<String> {
        if let Some(domain) = url.domain() {
            // Check all rules that matches this domain, but return on the first clean
            for domaininfo in self.cleaning_info.iter().filter(|&x| x.domain == domain) {
                if domaininfo.path == url.path() {
                    println!("{}", url);
                    println!("Discusting url, cleaning");
                    let pairs = url.query_pairs();
                    // First search all the queries for the link querykey
                    for (key, value) in pairs {
                        if key.as_ref() == domaininfo.querykey {
                            if let Ok(url) = Url::parse(&value) {
                                // Before returning, remove any click identifier as well
                                return Some(self.clean_query(&url).0.to_string());
                            }
                        }
                    }
                }
            }
            //println!("Url is clean");
            // Check if there is a click identifier, and return if there is one
            let (url, modified) = self.clean_query(&url);
            if modified {
                return Some(url.to_string());
            }
        }
        None
    }

    pub fn try_clean_string(&self, url_string: String) -> String {
        if let Ok(parsed) = Url::parse(&url_string) {
            if let Some(clean) = self.clean_url(&parsed) {
                return clean;
            }
        }

        url_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_facebook() {
        let youtube_dirty ="https://l.facebook.com/l.php?u=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DuBKajwUM5v4%26fbclid%3DIwAR0fqKqv6CeHBG0xbnI7KyYNSkFpGpVpfSynXjFXBPFQcErCqLRLgVbfYYw&h=AT01YUWDOjvNW9S09aDSRAZQZk6L55-JZGswiFa1SY6c8_mGQC0VMlNf4HXZhjdJH4PuqdNHctfOmMqISuBRBD10xZ_gIKCnwBGkAV3mrNdTtb7t6QMgyD0GzH3PSCPHmmZGyMBHCRjZ";
        let youtube_clean = "https://www.youtube.com/watch?v=uBKajwUM5v4";

        let parsed = Url::parse(&youtube_dirty).unwrap();
        let cleaner = UrlCleaner::default();
        let clean = cleaner.clean_url(&parsed).unwrap();

        assert_eq!(clean, youtube_clean);
    }

    #[test]
    fn clean_facebook2() {
        let url_dirty ="https://l.facebook.com/l.php?u=https%3A%2F%2Fwww.banggood.com%2FXT30-V3-ParaBoard-Parallel-Charging-Board-Banana-Plug-For-iMax-B6-Charger-p-1235388.html%3Fp%3DJQ191716342021201711%26custlinkid%3D37737%26fbclid%3DIwAR0ZRlKtl4NJgkCGMuiVNuxnL3GUVnw0kCLSmwNFD_xqiUv83U_dVP-6X8A&h=AT1jV6cBYrlCCqMs2RUB2mHXcyuSq4zO_1safL4SYIvxkwWVDs7xViyTB1dYm-84aACs8qfshYEHY0pS8o2H0cdRw51mK9ZQGmKZlodbgvCkZhs3v1LxumxDGCHcIey-8M1sLH1gXAN6";
        let url_clean = "https://www.banggood.com/XT30-V3-ParaBoard-Parallel-Charging-Board-Banana-Plug-For-iMax-B6-Charger-p-1235388.html?p=JQ191716342021201711";

        let parsed = Url::parse(&url_dirty).unwrap();
        let cleaner = UrlCleaner::default();
        let clean = cleaner.clean_url(&parsed).unwrap();

        assert_eq!(clean, url_clean);
    }
    #[test]
    fn clean_messenger() {
        let url_dirty ="https://l.messenger.com/l.php?u=https%3A%2F%2Fwww.reddit.com%2Fr%2FDnD%2Fcomments%2Fbzi1oq%2Fart_two_dragons_and_adopted_kobold_son%2F&h=AT3-avlfmolqmJ6-F1idHcFN3Mc6-qXDHj-IeV67w1ngQrk8M12v1UgS2sQnqaTxdFpoYKOoGH-JgwxojgF7g5dvIxamd6fWC2sSWuumpAcr9TZKwES5r5Fcq2U";
        let url_clean =
            "https://www.reddit.com/r/DnD/comments/bzi1oq/art_two_dragons_and_adopted_kobold_son/?";

        let parsed = Url::parse(&url_dirty).unwrap();
        let cleaner = UrlCleaner::default();
        let clean = cleaner.clean_url(&parsed).unwrap();

        assert_eq!(clean, url_clean);
    }

    #[test]
    fn clean_google_meeting() {
        let url = "https://www.google.com/url?q=https://meet.lync.com/skydrive3m-mmm/random/random&sa=D&ust=1560944361951000&usg=AOvVaw2hCRSIX_WKpRFxeczL2S0g";
        let url_clean = "https://meet.lync.com/skydrive3m-mmm/random/random?";
        let parsed = Url::parse(&url).unwrap();
        let cleaner = UrlCleaner::default();
        let clean = cleaner.clean_url(&parsed).unwrap();

        assert_eq!(clean, url_clean);
    }
    #[test]
    fn clean_facebook_image() {
        let url = "https://external.fbma2-1.fna.fbcdn.net/safe_image.php?d=AQBOrzUTFofcxXN7&w=960&h=960&url=https%3A%2F%2Fi.redd.it%2F4wao306sl9931.jpg&_nc_hash=AQDTUf7UFz8PtUsf";
        let url_clean = "https://i.redd.it/4wao306sl9931.jpg?";
        let parsed = Url::parse(&url).unwrap();
        let cleaner = UrlCleaner::default();
        let clean = cleaner.clean_url(&parsed).unwrap();

        assert_eq!(clean, url_clean);
    }
}
