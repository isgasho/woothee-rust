// This file is auto-generated! Any changes to this file will be lost!
extern crate woothee;

mod tests {
    use woothee::parser::Parser;

    #[test]
    fn test_crawler() {
        let parser = Parser::new();

        match parser.parse(r#"Mozilla/5.0 (compatible; Yahoo! Slurp; http://help.yahoo.com/help/us/ysearch/slurp)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; Yahoo! Slurp; http://help.yahoo.com/help/us/ysearch/slurp)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Slurp".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; Yahoo! Slurp/3.0; http://help.yahoo.com/help/us/ysearch/slurp)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; Yahoo! Slurp/3.0; http://help.yahoo.com/help/us/ysearch/slurp)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Slurp".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Y!J-BRO/YFSJ crawler (compatible; Mozilla 4.0; MSIE 5.5; http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html; YahooFeedSeekerJp/2.0; users 0; views 248)"#) {
            None => panic!(r#"invalid parse. "Y!J-BRO/YFSJ crawler (compatible; Mozilla 4.0; MSIE 5.5; http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html; YahooFeedSeekerJp/2.0; users 0; views 248)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Japan".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Y!J-BRP/YFSBJ crawler (compatible; Mozilla 4.0; MSIE 5.5; http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html; YahooFeedSeekerBetaJp/2.0; users 0; views 80)"#) {
            None => panic!(r#"invalid parse. "Y!J-BRP/YFSBJ crawler (compatible; Mozilla 4.0; MSIE 5.5; http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html; YahooFeedSeekerBetaJp/2.0; users 0; views 80)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Japan".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Y!J-BRJ/YATS crawler (http://listing.yahoo.co.jp/support/faq/int/other/other_001.html)"#) {
            None => panic!(r#"invalid parse. "Y!J-BRJ/YATS crawler (http://listing.yahoo.co.jp/support/faq/int/other/other_001.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Japan".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Y!J-BRJ/YATS crawler (http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html)"#) {
            None => panic!(r#"invalid parse. "Y!J-BRJ/YATS crawler (http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Japan".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Y!J-BSC/1.0 crawler (http://help.yahoo.co.jp/help/jp/blog-search/)"#) {
            None => panic!(r#"invalid parse. "Y!J-BSC/1.0 crawler (http://help.yahoo.co.jp/help/jp/blog-search/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Japan".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Linux; U; Android 2.3.3; ja-jp; sdk Build/GRI34; Y!J-BRZ/YATSHA crawler; http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Linux; U; Android 2.3.3; ja-jp; sdk Build/GRI34; Y!J-BRZ/YATSHA crawler; http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Japan".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (iPhone; Y!J-BRY/YATSH crawler; http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (iPhone; Y!J-BRY/YATSH crawler; http://help.yahoo.co.jp/help/jp/search/indexing/indexing-15.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Japan".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Yahoo Pipes 2.0"#) {
            None => panic!(r#"invalid parse. "Yahoo Pipes 2.0""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Yahoo! Pipes".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Baiduspider".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Baiduspider+(+http://www.baidu.jp/spider/)"#) {
            None => panic!(r#"invalid parse. "Baiduspider+(+http://www.baidu.jp/spider/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Baiduspider".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Baiduspider-image+(+http://www.baidu.com/search/spider.htm)"#) {
            None => panic!(r#"invalid parse. "Baiduspider-image+(+http://www.baidu.com/search/spider.htm)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Baiduspider".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"msnbot/1.1 (+http://search.msn.com/msnbot.htm)"#) {
            None => panic!(r#"invalid parse. "msnbot/1.1 (+http://search.msn.com/msnbot.htm)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "msnbot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"msnbot-UDiscovery/2.0b (+http://search.msn.com/msnbot.htm)"#) {
            None => panic!(r#"invalid parse. "msnbot-UDiscovery/2.0b (+http://search.msn.com/msnbot.htm)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "msnbot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"msnbot/2.0b (+http://search.msn.com/msnbot.htm)._"#) {
            None => panic!(r#"invalid parse. "msnbot/2.0b (+http://search.msn.com/msnbot.htm)._""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "msnbot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#""msnbot-NewsBlogs/2.0b (+http://search.msn.com/msnbot.htm)"#) {
            None => panic!(r#"invalid parse. ""msnbot-NewsBlogs/2.0b (+http://search.msn.com/msnbot.htm)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "msnbot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"msnbot-media/1.1 (+http://search.msn.com/msnbot.htm)"#) {
            None => panic!(r#"invalid parse. "msnbot-media/1.1 (+http://search.msn.com/msnbot.htm)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "msnbot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "bingbot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534+ (KHTML, like Gecko) BingPreview/1.0b"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534+ (KHTML, like Gecko) BingPreview/1.0b""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "BingPreview".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Yeti/1.0 (NHN Corp.; http://help.naver.com/robots/)"#) {
            None => panic!(r#"invalid parse. "Yeti/1.0 (NHN Corp.; http://help.naver.com/robots/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Naver Yeti".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/3.0 (compatible; Indy Library)"#) {
            None => panic!(r#"invalid parse. "Mozilla/3.0 (compatible; Indy Library)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Indy Library".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Apple-PubSub/65.28"#) {
            None => panic!(r#"invalid parse. "Apple-PubSub/65.28""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Apple iCloud".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"R6_CommentReader(www.radian6.com/crawler)"#) {
            None => panic!(r#"invalid parse. "R6_CommentReader(www.radian6.com/crawler)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "salesforce radian6".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"R6_FeedFetcher(www.radian6.com/crawler)"#) {
            None => panic!(r#"invalid parse. "R6_FeedFetcher(www.radian6.com/crawler)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "salesforce radian6".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; Genieo/1.0 http://www.genieo.com/webfilter.html)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; Genieo/1.0 http://www.genieo.com/webfilter.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Genieo Web Filter".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; Butterfly/1.0; +http://labs.topsy.com/butterfly/) Gecko/2009032608 Firefox/3.0.8"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; Butterfly/1.0; +http://labs.topsy.com/butterfly/) Gecko/2009032608 Firefox/3.0.8""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "topsy Butterfly".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"rogerbot/1.0 (http://www.seomoz.org/dp/rogerbot, rogerbot-crawler@seomoz.org)"#) {
            None => panic!(r#"invalid parse. "rogerbot/1.0 (http://www.seomoz.org/dp/rogerbot, rogerbot-crawler@seomoz.org)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "SeoMoz rogerbot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"rogerbot/1.0 (http://www.seomoz.org/dp/rogerbot, rogerbot-crawler+shiny@seomoz.org)"#) {
            None => panic!(r#"invalid parse. "rogerbot/1.0 (http://www.seomoz.org/dp/rogerbot, rogerbot-crawler+shiny@seomoz.org)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "SeoMoz rogerbot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Mozilla/5.0 (compatible; AhrefsBot/4.0; +http://ahrefs.com/robot/)"#) {
            None => panic!(r#"invalid parse. "Mozilla/5.0 (compatible; AhrefsBot/4.0; +http://ahrefs.com/robot/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "ahref AhrefsBot".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Hatena Antenna/0.5 (http://a.hatena.ne.jp/help)"#) {
            None => panic!(r#"invalid parse. "Hatena Antenna/0.5 (http://a.hatena.ne.jp/help)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Hatena".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Hatena Pagetitle Agent/1.0"#) {
            None => panic!(r#"invalid parse. "Hatena Pagetitle Agent/1.0""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Hatena".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Hatena Diary RSS Module (http://d.hatena.ne.jp/help#rssmodule)"#) {
            None => panic!(r#"invalid parse. "Hatena Diary RSS Module (http://d.hatena.ne.jp/help#rssmodule)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "Hatena".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"ichiro/3.0 (http://help.goo.ne.jp/door/crawler.html)"#) {
            None => panic!(r#"invalid parse. "ichiro/3.0 (http://help.goo.ne.jp/door/crawler.html)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "goo".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"DoCoMo/2.0 P900i(c100;TB;W24H11) (compatible; ichiro/mobile goo; +http://help.goo.ne.jp/help/article/1142/)"#) {
            None => panic!(r#"invalid parse. "DoCoMo/2.0 P900i(c100;TB;W24H11) (compatible; ichiro/mobile goo; +http://help.goo.ne.jp/help/article/1142/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "goo".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"gooblogsearch/2.0 (http://search.goo.ne.jp/option/use/sub4/sub4-1/)"#) {
            None => panic!(r#"invalid parse. "gooblogsearch/2.0 (http://search.goo.ne.jp/option/use/sub4/sub4-1/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "goo".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"livedoor FeedFetcher/0.01 (http://reader.livedoor.com/; 999 subscribers)"#) {
            None => panic!(r#"invalid parse. "livedoor FeedFetcher/0.01 (http://reader.livedoor.com/; 999 subscribers)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "livedoor FeedFetcher".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Fastladder FeedFetcher/0.01 (http://fastladder.com/; 27 subscribers)"#) {
            None => panic!(r#"invalid parse. "Fastladder FeedFetcher/0.01 (http://fastladder.com/; 27 subscribers)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "livedoor FeedFetcher".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"Twitterbot/1.0"#) {
            None => panic!(r#"invalid parse. "Twitterbot/1.0""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "twitter".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php)"#) {
            None => panic!(r#"invalid parse. "facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "facebook".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"mixi-check/1.0 (http://mixi.jp/)"#) {
            None => panic!(r#"invalid parse. "mixi-check/1.0 (http://mixi.jp/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "mixi".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"mixi-news-crawler/1.00 (http://mixi.jp/)"#) {
            None => panic!(r#"invalid parse. "mixi-news-crawler/1.00 (http://mixi.jp/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "mixi".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
        match parser.parse(r#"mixi-crawler/2.00 (http://mixi.jp/)"#) {
            None => panic!(r#"invalid parse. "mixi-crawler/2.00 (http://mixi.jp/)""#),
            Some(result) => {
                assert_eq!(result.category, "crawler".to_string());
                assert_eq!(result.name, "mixi".to_string());
                assert_eq!(result.os, "UNKNOWN".to_string());
                assert_eq!(result.os_version, "UNKNOWN".to_string());
                assert_eq!(result.version, "UNKNOWN".to_string());
            }
        }
    }
}