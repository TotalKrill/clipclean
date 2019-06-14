use url::*;

pub struct CleanInformation<'a> {
    domain: &'a str,
    path: &'a str,
    querykey: &'a str,
}

const KEYS_TO_CLEAN: [&'static str; 3] = {[
"fbclid",
"custlinkid",
"gclid",
]};

const DOMAINS_TO_CLEAN: [ CleanInformation<'static>; 3 ] = {[
            CleanInformation {domain: "l.facebook.com", path: "/l.php", querykey: "u"},
            CleanInformation {domain: "l.messenger.com", path: "/l.php", querykey: "u"},
            CleanInformation {domain: "www.google.com", path: "/url", querykey: "q"},
]};


// remove the click-id and similar query that can sometimes come hidden inside links
pub fn clean_query<'a>( url: &url::Url ) -> url::Url {
    let pairs = url.query_pairs();
    let mut newurl = url.clone();
    newurl.query_pairs_mut().clear();

    for (key, value) in pairs {
        if KEYS_TO_CLEAN.contains(&key.as_ref()) {
            println!("key found: {:?}", key);
        } else {
            newurl.query_pairs_mut().append_pair(&key, &value);
        }
    }
    newurl
}


/// try to extract the destination url from the link if possible and also try to remove the click-id
/// query parameters that are available
pub fn clean_url<'a>( url: &url::Url ) -> Option<String> {

    if let Some(domain) = url.domain() {
	if let Some(domaininfo) = DOMAINS_TO_CLEAN.iter().find(|&x| x.domain == domain)
	{
	    if domaininfo.path == url.path() {
                println!("{}", url);
	        println!("Discusting url, cleaning");
		let pairs = url.query_pairs();
		for (key, value) in pairs {
		    if key == domaininfo.querykey {
			if let Ok(url) = Url::parse(&value) {
			    return Some(clean_query(&url).to_string());
			}
		    }
		}
	    }
        }
	else {
            //println!("Url is clean");
	    Some( clean_query(&url).to_string() );
	}
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_facebook() {
        let youtube_dirty ="https://l.facebook.com/l.php?u=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DuBKajwUM5v4%26fbclid%3DIwAR0fqKqv6CeHBG0xbnI7KyYNSkFpGpVpfSynXjFXBPFQcErCqLRLgVbfYYw&h=AT01YUWDOjvNW9S09aDSRAZQZk6L55-JZGswiFa1SY6c8_mGQC0VMlNf4HXZhjdJH4PuqdNHctfOmMqISuBRBD10xZ_gIKCnwBGkAV3mrNdTtb7t6QMgyD0GzH3PSCPHmmZGyMBHCRjZ";
        let youtube_clean = "https://www.youtube.com/watch?v=uBKajwUM5v4";

        let parsed = Url::parse(&youtube_dirty).unwrap();
        let clean = clean_url(&parsed).unwrap();

        assert_eq!(clean, youtube_clean);

    }

    #[test]
    fn clean_facebook2() {
        let url_dirty ="https://l.facebook.com/l.php?u=https%3A%2F%2Fwww.banggood.com%2FXT30-V3-ParaBoard-Parallel-Charging-Board-Banana-Plug-For-iMax-B6-Charger-p-1235388.html%3Fp%3DJQ191716342021201711%26custlinkid%3D37737%26fbclid%3DIwAR0ZRlKtl4NJgkCGMuiVNuxnL3GUVnw0kCLSmwNFD_xqiUv83U_dVP-6X8A&h=AT1jV6cBYrlCCqMs2RUB2mHXcyuSq4zO_1safL4SYIvxkwWVDs7xViyTB1dYm-84aACs8qfshYEHY0pS8o2H0cdRw51mK9ZQGmKZlodbgvCkZhs3v1LxumxDGCHcIey-8M1sLH1gXAN6";
        let url_clean = "https://www.banggood.com/XT30-V3-ParaBoard-Parallel-Charging-Board-Banana-Plug-For-iMax-B6-Charger-p-1235388.html?p=JQ191716342021201711";

        let parsed = Url::parse(&url_dirty).unwrap();
        let clean = clean_url(&parsed).unwrap();

        assert_eq!(clean, url_clean);

    }
    #[test]
    fn clean_messenger() {
        let url_dirty ="https://l.messenger.com/l.php?u=https%3A%2F%2Fwww.reddit.com%2Fr%2FDnD%2Fcomments%2Fbzi1oq%2Fart_two_dragons_and_adopted_kobold_son%2F&h=AT3-avlfmolqmJ6-F1idHcFN3Mc6-qXDHj-IeV67w1ngQrk8M12v1UgS2sQnqaTxdFpoYKOoGH-JgwxojgF7g5dvIxamd6fWC2sSWuumpAcr9TZKwES5r5Fcq2U";
        let url_clean = "https://www.reddit.com/r/DnD/comments/bzi1oq/art_two_dragons_and_adopted_kobold_son/?";

        let parsed = Url::parse(&url_dirty).unwrap();
        let clean = clean_url(&parsed).unwrap();

        assert_eq!(clean, url_clean);

    }

    #[test]
    fn clean_google_meeting() {
        let url = "https://www.google.com/url?q=https://meet.lync.com/skydrive3m-mmm/random/random&sa=D&ust=1560944361951000&usg=AOvVaw2hCRSIX_WKpRFxeczL2S0g";
        let url_clean = "https://meet.lync.com/skydrive3m-mmm/random/random?";
        let parsed = Url::parse(&url).unwrap();
        let clean = clean_url(&parsed).unwrap();

        assert_eq!(clean, url_clean);

    }
}
