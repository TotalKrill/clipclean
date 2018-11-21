use url::*;
use url::percent_encoding::percent_decode;

pub fn clean_urlold<'a>( url: &url::Url ) -> Option<String> {

    if let Some(domain) = url.domain() {
        let should_clean = match domain {
            "l.facebook.com" => true,
            _ => false,
        };
        if should_clean {

            println!("{}", url);
            println!("Discusting url, cleaning");
            if let Some(query) = url.query() {
                let decoded = percent_decode(query.as_bytes()).decode_utf8();
                if let Ok(decoded) = decoded {
                    let len = decoded.len();
                    let u = &decoded[2..len];

                    let parsed = Url::parse(&u);
                    if let Ok(parsed) = parsed {
                        let cleaned: &str = &parsed[..Position::AfterQuery];
                        return Some(cleaned.into());
                    }
                } else {
                    println!("Could not percent decode");
                }
            }
        }
    }
    None
}

const keys_to_clean: [&'static str; 3] = {[
"fbclid",
"custlinkid",
"gclid",
]};

pub fn clean_query<'a>( url: &url::Url ) -> url::Url {
    let pairs = url.query_pairs();
    let mut newurl = url.clone();
    newurl.query_pairs_mut().clear();

    for (key, value) in pairs {
        if keys_to_clean.contains(&key.as_ref()) {
            println!("key found: {:?}", key);
        } else {
            println!("key not found: {:?}", key);
            newurl.query_pairs_mut().append_pair(&key, &value);
        }
    }
    println!("new: {:?}", newurl);
    newurl
}

pub fn clean_url<'a>( url: &url::Url ) -> Option<String> {

    if let Some(domain) = url.domain() {
        let should_clean = match domain {
            "l.facebook.com" => true,
            _ => false,
        };
        if should_clean {
            let pairs = url.query_pairs();
            for (key, value) in pairs {
                if key == "u" {
                    println!("{:?}", value);
                    if let Ok(mut url) = Url::parse(&value) {
                        println!("{}", url);
                        println!("Discusting url, cleaning");
                        return Some(clean_query(&url).to_string());
                    }
                }

            }
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_links() {
        let youtube_dirty ="https://l.facebook.com/l.php?u=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DuBKajwUM5v4%26fbclid%3DIwAR0fqKqv6CeHBG0xbnI7KyYNSkFpGpVpfSynXjFXBPFQcErCqLRLgVbfYYw&h=AT01YUWDOjvNW9S09aDSRAZQZk6L55-JZGswiFa1SY6c8_mGQC0VMlNf4HXZhjdJH4PuqdNHctfOmMqISuBRBD10xZ_gIKCnwBGkAV3mrNdTtb7t6QMgyD0GzH3PSCPHmmZGyMBHCRjZ";
        let youtube_clean = "https://www.youtube.com/watch?v=uBKajwUM5v4";

        let parsed = Url::parse(&youtube_dirty).unwrap();
        let clean = clean_url(&parsed).unwrap();

        assert_eq!(clean, youtube_clean);

    }

    #[test]
    fn clean_link2() {
        let url_dirty ="https://l.facebook.com/l.php?u=https%3A%2F%2Fwww.banggood.com%2FXT30-V3-ParaBoard-Parallel-Charging-Board-Banana-Plug-For-iMax-B6-Charger-p-1235388.html%3Fp%3DJQ191716342021201711%26custlinkid%3D37737%26fbclid%3DIwAR0ZRlKtl4NJgkCGMuiVNuxnL3GUVnw0kCLSmwNFD_xqiUv83U_dVP-6X8A&h=AT1jV6cBYrlCCqMs2RUB2mHXcyuSq4zO_1safL4SYIvxkwWVDs7xViyTB1dYm-84aACs8qfshYEHY0pS8o2H0cdRw51mK9ZQGmKZlodbgvCkZhs3v1LxumxDGCHcIey-8M1sLH1gXAN6";
        let url_clean = "https://www.banggood.com/XT30-V3-ParaBoard-Parallel-Charging-Board-Banana-Plug-For-iMax-B6-Charger-p-1235388.html?p=JQ191716342021201711";

        let parsed = Url::parse(&url_dirty).unwrap();
        let clean = clean_url(&parsed).unwrap();

        assert_eq!(clean, url_clean);

    }
}
