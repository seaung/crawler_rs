use scraper::{Html, Selector};
use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = "https://quotes.toscrape.com".to_string();

    loop {
        // 获取 HTML 内容
        let html = reqwest::blocking::get(&url)?.text()?;
        let document = Html::parse_document(&html);

        // 定义选择器
        let quote_selector = Selector::parse(".quote").unwrap();
        let text_selector = Selector::parse(".text").unwrap();
        let author_selector = Selector::parse(".author").unwrap();
        let tags_selector = Selector::parse(".tags .tag").unwrap();
        let next_selector = Selector::parse(".pager .next a").unwrap();

        // 提取当前页数据
        for quote in document.select(&quote_selector) {
            let text = quote.select(&text_selector).next().map(|el| el.inner_html()).unwrap_or_default();
            let author = quote.select(&author_selector).next().map(|el| el.inner_html()).unwrap_or_default();
            let tags: Vec<String> = quote.select(&tags_selector).map(|el| el.inner_html()).collect();

            println!("名言: {}", text);
            println!("作者: {}", author);
            println!("主题标签: {:?}", tags);
            println!("---");
        }

        // 检查是否有下一页
        if let Some(next_page) = document.select(&next_selector).next() {
            if let Some(next_href) = next_page.value().attr("href") {
                url = format!("https://quotes.toscrape.com{}", next_href);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(())
}