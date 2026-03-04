// In src/page_types/post.rs
use crate::prelude::*;

struct CtaContent {
    headline: &'static str,
    subtext: &'static str,
    sticky_text: &'static str,
    mid_cta_text: &'static str,
}

const DENTAL_CTA: CtaContent = CtaContent {
    headline: "Need Emergency Dental Care?",
    subtext: "Same-day appointments from just £20. Open 24 hours, 7 days a week.",
    sticky_text: "Got a dental concern? We're here 24/7.",
    mid_cta_text: "Worried about a dental problem?",
};

const HAIR_CTA: CtaContent = CtaContent {
    headline: "Considering a Hair Transplant?",
    subtext: "Natural results from just £2,500. Book a free consultation or call us now.",
    sticky_text: "Considering hair restoration? We're here to help.",
    mid_cta_text: "Thinking about a hair transplant?",
};

const COSMETIC_CTA: CtaContent = CtaContent {
    headline: "Ready for Your Dream Smile?",
    subtext: "Composite bonding from £299, veneers from £695. Book a consultation or call us now.",
    sticky_text: "Want to transform your smile? We're here to help.",
    mid_cta_text: "Thinking about a smile makeover?",
};

const ALIGNERS_CTA: CtaContent = CtaContent {
    headline: "Want Straighter Teeth?",
    subtext: "Clear aligners from £2,999 with a free consultation. Book online or call us now.",
    sticky_text: "Thinking about straighter teeth? We're here to help.",
    mid_cta_text: "Thinking about teeth straightening?",
};

const PHONE_SVG_16: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z"/></svg>"#;

const PHONE_SVG_14: &str = r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2z"/></svg>"#;

fn match_cta_variant(tags: &[String]) -> (&'static str, &'static CtaContent) {
    for tag in tags {
        match tag.as_str() {
            "Hair Transplants" => return ("hair", &HAIR_CTA),
            "Clear Aligners" | "Orthodontics" | "Invisalign" | "Braces" => return ("aligners", &ALIGNERS_CTA),
            "Cosmetic Dentistry" | "Cosmetic Treatments" | "Veneers"
            | "Composite Bonding" | "Teeth Whitening" | "Smile Makeover"
            | "Gummy Smile" | "Dental Bonding" | "Smile Journey" => return ("cosmetic", &COSMETIC_CTA),
            _ => {}
        }
    }
    ("dental", &DENTAL_CTA)
}

pub fn declare_post_sections(site: &mut Site<UCDPages>) {
    let variants: &[(&str, &CtaContent)] = &[
        ("dental", &DENTAL_CTA),
        ("hair", &HAIR_CTA),
        ("cosmetic", &COSMETIC_CTA),
        ("aligners", &ALIGNERS_CTA),
    ];

    for (key, cta) in variants {
        site.declare_section(
            &format!("post_cta_{key}"),
            &format!(
                r##"<div class="post-cta">
                    <h2>{headline}</h2>
                    <p>{subtext}</p>
                    <div class="buttons">
                        <a href="{phone_link}" class="primary">{phone_svg} Call {phone}</a>
                        <a href="{booking_link}" class="secondary">Book Online</a>
                    </div>
                </div>"##,
                headline = cta.headline,
                subtext = cta.subtext,
                phone_link = PHONE_NUMBER_LINK,
                phone = PHONE_NUMBER,
                phone_svg = PHONE_SVG_16,
                booking_link = BOOKING_LINK,
            ),
        );

        site.declare_section(
            &format!("sticky_bar_{key}"),
            &format!(
                r##"<div class="sticky-bar" id="sticky-bar">
                    <span>{sticky_text}</span>
                    <div class="sticky-buttons">
                        <a href="{phone_link}" class="call">{phone_svg} Call Now</a>
                        <a href="{booking_link}">Book Online</a>
                    </div>
                </div>
                <script>
                (function() {{
                    var bar = document.getElementById('sticky-bar');
                    var shown = false;
                    window.addEventListener('scroll', function() {{
                        if (window.scrollY > 600 && !shown) {{
                            bar.classList.add('visible');
                            shown = true;
                        }}
                    }});
                }})();
                </script>"##,
                sticky_text = cta.sticky_text,
                phone_link = PHONE_NUMBER_LINK,
                phone_svg = PHONE_SVG_14,
                booking_link = BOOKING_LINK,
            ),
        );
    }
}

pub fn construct_post(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    let post = match page.specification.clone() {
        UCDPages::BlogPost(post) => post,
        _ => return,
    };

    // Add schema for this blog post
    add_blog_post_schema(page, &post);
    add_post_og_image(page, &post);

    css(site);

    let head = site.construct_head(page);
    let date = format_date(&post.frontmatter.date);

    // Get the first category for breadcrumbs
    /*let category = post.frontmatter.category
        .first()
        .map(|c| c.as_str())
        .unwrap_or("Uncategorized");

    let category_slug = slugify(category);*/
    /*let author = if !post.frontmatter.author.is_empty() {
        format!(" by {}", post.frontmatter.author)
    } else {
        String::new()
    };*/

    // Build meta HTML with both date and updated
    let meta_html = if date.is_empty() && post.frontmatter.updated.is_empty() {
        String::new()
    } else {
        let updated_html = if !post.frontmatter.updated.is_empty() {
            let updated_date = format_date(&post.frontmatter.updated);
            format!(r##"<div class="updated">Last Updated: {updated_date}</div>"##)
        } else {
            String::new()
        };

        format!(
            r##"
            <div class="meta">
                <div class="published">Published: {date}</div>
                {updated_html}
            </div>
        "##
        )
    };

    let author_card = format!(
        r##"
        <div class="author-card">
            <div class="image-box">
                <img src="/images/branding/UrgentCare-Dental-Logo.svg" alt="Urgent Care Dental Logo">
            </div>
            <div class="text-area">
                <p class="author">UCD Editorial Team</p>
                <p class="role">Department of Dentistry Journalism</p>
                <p class="organization">UrgentCare Dental</p>
            </div>
        </div>
    "##
    );

    let tags_html = post
        .frontmatter
        .tags
        .iter()
        .take(3)
        .map(|tag| format!(r##"<span class="tag">{}</span>"##, tag))
        .collect::<Vec<_>>()
        .join("");

    let unsplash_attribution = if !post.frontmatter.unsplash_author.is_empty() {
        format!(
            r##"<div class="unsplash-attribution">Photo by <a href="{url}?utm_source=urgentcare_dental_blog&utm_medium=referral" target="_blank" rel="noopener">{author}</a> on <a href="https://unsplash.com/?utm_source=urgentcare_dental_blog&utm_medium=referral" target="_blank" rel="noopener">Unsplash</a></div>"##,
            author = post.frontmatter.unsplash_author,
            url = post.frontmatter.unsplash_author_url,
        )
    } else {
        String::new()
    };

    let (variant_key, variant_cta) = match_cta_variant(&post.frontmatter.tags);
    let post_cta = site.sections[&format!("post_cta_{variant_key}")].clone();
    let sticky_bar = site.sections[&format!("sticky_bar_{variant_key}")].clone();

    let html = format!(
        r##"
        <!DOCTYPE html>
        <html lang="en-GB">
        {head}
        <body>
            {header}
            <main class="post">
            <div class="polka-dots"></div>
            <div class="background-fade"></div>
                <div class="inner">
                    <article class="content">
                        <div class="meta-area">
                            <div class="breadcrumbs">
                                <a href="/">Home</a>
                                <span class="separator">›</span>
                                <a href="/blog">Blog</a>
                            </div>
                            {meta_html}
                        </div>
                        {author_card}
                        <h1>{title}</h1>
                        <div class="main-image">
                            <img src="{image}" alt="{title}">
                            {unsplash_attribution}
                        </div>
                        <div class="tags">
                            {tags_html}
                        </div>
                        <div class="content">
                            {content}
                        </div>
                        {post_cta}
                    </article>
                </div>
            </main>
            {sticky_bar}
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
        title = post.frontmatter.title,
        meta_html = meta_html,
        image = post.frontmatter.image,
        unsplash_attribution = unsplash_attribution,
        content = inject_mid_cta(&post.content, variant_cta),
        post_cta = post_cta,
        sticky_bar = sticky_bar,
    );

    page.foundation.content = Some(html);
}

fn inject_mid_cta(content: &str, cta: &CtaContent) -> String {
    let mid_cta = format!(
        r##"<div class="inline-cta"><span>{}</span> <a href="{}">Call us on {}</a> for a free consultation.</div>"##,
        cta.mid_cta_text, PHONE_NUMBER_LINK, PHONE_NUMBER
    );

    // Find a </p> near the midpoint to insert after
    let midpoint = content.len() / 2;
    let search_region = &content[midpoint..];

    if let Some(offset) = search_region.find("</p>") {
        let insert_at = midpoint + offset + 4; // after </p>
        let mut result = content[..insert_at].to_owned();
        result.push_str(&mid_cta);
        result.push_str(&content[insert_at..]);
        result
    } else {
        content.to_owned()
    }
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("post", r##"
    
    {}
        
        main.post {
            
            position: relative;
            margin: 0 auto;
            
            
            .polka-dots {
                position: absolute;
                width: 100%;
                height: 100%;
                max-height: 200vh;
                background-color: transparent;
                background-image: radial-gradient(rgb(2, 220, 227) 1px, transparent 1px), radial-gradient(rgb(2, 220, 227) 1px, rgba(35, 84, 84, 0) 1px);
                background-position: 0px 0px, 20px 20px;
                background-size: 40px 40px;
                border-radius: 0px;
                z-index: -2;
            }
            
            .background-fade {
                position: absolute;
                z-index: -1;
                width: 100%;
                height: 100%;
                max-height: 200vh;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: linear-gradient(150deg, rgba(255, 255, 255, 0) 0%, rgb(255, 255, 255) 70%);
            }
            
            
            .inner {
            
                padding: 100px var(--site-padding-x) 100px;
                z-index: 10;
                margin: 0 auto;
                box-sizing: border-box;
                
                @media (max-width: 768px) {
                
                    padding: 100px 12px 100px;
                
                }

                article.content {
                
                    z-index: 10;
                    margin: 0 auto;
                    max-width: 702px;
                    padding: 0;
                    
                    
                    .meta-area {
                    
                        margin-bottom: 32px;
                        display: flex;
                        justify-content: space-between;
                        font-size: 14px;
                        
                        
                        
                            .breadcrumbs {
                                
                                /*border-bottom: 1px solid #eee;*/
                                z-index: 10;
                                
                                a {
                                    color: var(--link-color);
                                    text-decoration: none;
                                    
                                    &:hover {
                                        color: var(--link-hover-color);
                                        text-decoration: underline;
                                    }
                                }
                                
                                .separator {
                                    margin: 0 10px;
                                    color: #999;
                                }
                            
                            }
                        
                            .meta {
                                color: #666;
                            
                            
                            .published {
                                margin-bottom: 5px;
                            }
                            
                            .updated {
                                font-size: 14px;
                                color: #999;
                                font-style: italic;
                            }
                        }
                        
                        
                    
                    
                    }
                
                    .author-card {
                    
                        display: flex;
                        background-color: rgb(230, 230, 230);
                        border-radius: 100px;
                        align-items: center;
                        padding: 8px 12px;
                        gap: 18px;
                        width: fit-content;
                        margin-bottom: 28px;
                    
                        .image-box {
                        
                            width: 60px;
                            height: 60px;
                            border-radius: 50%;
                            background-color: white;
                            padding: 8px;
                            box-sizing: border-box;
                            place-content: center;
                            
                            img {
                            
                                padding-top: 3px;
                            
                            }
                        }
                        
                        .text-area {
                        
                            display: flex;
                            flex-direction: column;
                            padding: 0 12px 0 0;
                            gap: 4px;
                        
                            p {
                            
                                margin: 0;
                                line-height: 1.2em;
                                
                                
                                &.author {
                            
                                    font-size: 16px;
                                    
                                }
                                
                                &.role {
                                
                                    font-size: 14px;
                                
                                }
                                
                                &.organization {
                                
                                    font-size: 12px;
                                
                                }
                            
                            }
                        
                            
                        
                            
                        
                        
                        }
                        
                    
                    }
                    
                    .main-image {

                        margin-bottom: 12px;

                        .unsplash-attribution {
                            font-size: 12px;
                            color: #999;
                            margin-top: 6px;

                            a {
                                color: #999;
                                text-decoration: underline;

                                &:hover {
                                    color: #666;
                                }
                            }
                        }

                    }
                    
                    .tags {
                                
                        place-content: center flex-start;
                        align-items: center;
                        display: flex;
                        flex: 0 0 auto;
                        flex-flow: wrap;
                        gap: 12px;
                        height: min-content;
                        overflow: visible;
                        padding: 0px;
                        position: relative;
                        width: 100%;
                        margin-bottom: 48px;
                        
                        
                        .tag {
                        
                            font-size: 14px;
                            background-color:rgb(204, 255, 255);
                            border-radius: 2px;
                            opacity: 1;
                            color: var(--turquoise-15);
                            place-content: center;
                            align-items: center;
                            display: flex;
                            flex-flow: row;
                            gap: 10px;
                            height: min-content;
                            overflow: visible;
                            padding: 2px 12px;
                            position: relative;
                        }
                    }
                
                
                    h1 {
                        font-size: 46px;
                        font-weight: 600;
                        line-height: 1.2;
                        margin-bottom: 48px;
                        color: var(--turquoise-15);
                        
                        @media (max-width: 768px) {
                            font-size: 30px;
                        }
                    }
                    
                    .post-cta {
                        margin-top: 60px;
                        background: linear-gradient(135deg, #029297, #017275);
                        border-radius: 12px;
                        padding: 48px 40px;
                        text-align: center;

                        h2 {
                            color: #fff;
                            font-size: 32px;
                            font-weight: 600;
                            margin: 0 0 12px;
                        }

                        p {
                            color: rgba(255, 255, 255, 0.9);
                            font-size: 18px;
                            margin: 0 0 28px;
                        }

                        .buttons {
                            display: flex;
                            gap: 16px;
                            justify-content: center;
                            flex-wrap: wrap;

                            a {
                                padding: 16px 36px;
                                border-radius: 8px;
                                font-size: 17px;
                                font-weight: 600;
                                text-decoration: none;
                                transition: transform 0.15s ease, box-shadow 0.15s ease;
                                display: inline-flex;
                                align-items: center;
                                justify-content: center;
                                line-height: 1;
                                box-sizing: border-box;

                                &:hover {
                                    transform: translateY(-2px);
                                    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
                                    text-decoration: none;
                                }

                                svg { margin-right: 8px; display: block; }

                                &.primary {
                                    background: #fff;
                                    color: #029297;
                                    font-size: 19px;
                                    padding: 18px 40px;
                                    border: 2px solid #fff;
                                }

                                &.secondary {
                                    background: transparent;
                                    color: #fff;
                                    border: 2px solid rgba(255, 255, 255, 0.6);
                                    font-size: 15px;
                                }
                            }
                        }

                        @media (max-width: 768px) {
                            padding: 36px 24px;

                            h2 { font-size: 24px; }

                            .buttons {
                                flex-direction: column;

                                a { width: 100%; box-sizing: border-box; }
                            }
                        }
                    }

                    .content {
                        font-size: 18px;
                        line-height: 1.625;
                        color: rgb(35, 35, 35);
                        letter-spacing: -0.18px;

                        .inline-cta {
                            background: var(--turquoise-98, #f0fafa);
                            border-left: 4px solid #029297;
                            padding: 18px 24px;
                            margin: 32px 0;
                            border-radius: 0 8px 8px 0;
                            font-size: 17px;
                            line-height: 1.5;

                            a {
                                color: #029297;
                                font-weight: 600;
                                text-decoration: none;
                                white-space: nowrap;

                                &:hover {
                                    text-decoration: underline;
                                }
                            }
                        }
                        
                        p {
                        
                            margin: 0 0 28px;
                            
                            &:last-child {
                                margin-bottom: 0;
                            }
                            
                        }
                        
                        h2 {
                            font-size: 36px;
                            font-weight: 600;
                            line-height: 1.2;
                            margin: 40px 0 20px;
                            color: var(--turquoise-15);
                            
                            @media (max-width: 768px) {
                                font-size: 32px;
                            }
                        }
                        
                        h3 {
                            font-size: 24px;
                            font-weight: 600;
                            line-height: 1.2;
                            margin: 30px 0 20px;
                            color: var(--turquoise-15);
                        }
                        
                        h4 {
                            font-size: 20px;
                            font-weight: 600;
                            line-height: 1.2;
                            margin: 25px 0 15px;
                            color: var(--turquoise-15);
                        }
                        
                        a {
                            color: var(--link-color);
                            transition: color 0.2s ease-in-out;
                            text-decoration: none;
                            
                            &:hover {
                                color: var(--link-hover-color);
                                text-decoration: underline;
                            }

                        }
                        
                        img {
                            max-width: 100%;
                            height: auto;
                            display: block;
                            margin: 30px auto;
                            border-radius: 3px;
                        }
                        
                        blockquote {
                            margin: 30px 40px;
                            padding: 20px 30px;
                            background-color: #f5f5f5;
                            border-left: 4px solid var(--link-color);
                            font-style: italic;
                            
                            p:last-child {
                                margin-bottom: 0;
                            }
                        }
                        
                        ul, ol {
                            margin: 0 0 30px;
                            padding-left: 40px;
                            
                            li {
                                margin-bottom: 10px;
                                line-height: 1.625;
                            }
                        }
                        
                        ul li {
                            list-style-type: disc;
                        }
                        
                        ol li {
                            list-style-type: decimal;
                        }
                        
                        code {
                            background-color: #f5f5f5;
                            padding: 2px 6px;
                            border-radius: 3px;
                            font-family: 'Monaco', 'Menlo', monospace;
                            font-size: 0.9em;
                        }
                        
                        pre {
                            background-color: #f5f5f5;
                            padding: 20px;
                            border-radius: 5px;
                            overflow-x: auto;
                            margin: 30px 0;
                            
                            code {
                                background: none;
                                padding: 0;
                            }
                        }
                        
                        hr {
                            border: 0;
                            border-bottom: 1px solid #eee;
                            margin: 40px auto;
                            clear: both;
                        }
                        
                        table {
                            width: 100%;
                            border-collapse: collapse;
                            margin: 30px 0;
                            overflow-x: auto;
                            position: relative;
                            
                            tr {
                                background-color: #fff;
                                word-break: normal;
                                
                                &:nth-child(even) {
                                    background-color: var(--turquoise-98);
                                }
                            }
                            
                            tbody:only-child {
                                tr {
                                    &:nth-child(odd) {
                                        background-color: var(--turquoise-98);
                                    }
                                    
                                    &:nth-child(even) {
                                        background-color: #fff;
                                    }
                                    
                                    &:first-child {
                                        background-color: var(--turquoise-30);
                                        color: #fff;
                                        
                                        td {
                                            font-weight: 600;
                                        }
                                    }
                                }
                            }
                            
                            th {
                                background-color: var(--turquoise-30);
                                font-weight: 600;
                                color: #fff;
                                text-align: left;
                                padding: 12px;
                                border: 1px solid var(--turquoise-15);
                            }
                            
                            td {
                                padding: 12px;
                                border: 1px solid var(--turquoise-90);
                                text-align: left;
                                vertical-align: middle;
                            }
                        }
                        
                        .alignleft {
                            float: left;
                            margin: 0 20px 20px 0;
                        }
                        
                        .alignright {
                            float: right;
                            margin: 0 0 20px 20px;
                        }
                        
                        .aligncenter {
                            display: block;
                            margin: 30px auto;
                            text-align: center;
                        }
                        
                        figure {
                            margin: 30px 0;
                            
                            img {
                                margin-bottom: 10px;
                            }
                            
                            figcaption {
                                font-size: 14px;
                                color: #666;
                                text-align: center;
                                font-style: italic;
                                padding: 0 10px;
                            }
                        }
                    }
                
                
                
                
                }
            }
            
           
        }

        @media (max-width: 768px) {
            .content-sidebar-wrap {
                padding: 0 20px;
                margin-top: 30px;
            }
            
            main.post {

                max-width: 100%;

                h1 {
                    font-size: 24px;
                }
                
                .content {
                    font-size: 16px;
                    
                    h2 {
                        font-size: 22px;
                    }
                    
                    h3 {
                        font-size: 20px;
                    }
                    
                    h4 {
                        font-size: 18px;
                    }
                    
                    blockquote {
                        margin: 20px 20px;
                        padding: 15px 20px;
                    }
                    
                    ul, ol {
                        padding-left: 30px;
                    }
                    
                    .alignleft, .alignright {
                        float: none;
                        display: block;
                        margin: 20px auto;
                    }
                }
            }
        }

        .sticky-bar {
            position: fixed;
            bottom: 0;
            left: 0;
            right: 0;
            background: #029297;
            color: #fff;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 20px;
            padding: 12px 24px;
            z-index: 1000;
            transform: translateY(100%);
            transition: transform 0.3s ease;
            box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.15);

            &.visible {
                transform: translateY(0);
            }

            span {
                font-size: 15px;
                font-weight: 500;
            }

            .sticky-buttons {
                display: flex;
                gap: 10px;
                align-items: center;

                a {
                    padding: 9px 20px;
                    border-radius: 6px;
                    font-size: 14px;
                    font-weight: 600;
                    line-height: 1;
                    text-decoration: none;
                    background: transparent;
                    color: #fff;
                    border: 1.5px solid rgba(255, 255, 255, 0.5);
                    white-space: nowrap;
                    box-sizing: border-box;
                    display: inline-flex;
                    align-items: center;
                    justify-content: center;

                    &.call {
                        background: #fff;
                        color: #029297;
                        border: 1.5px solid #fff;

                        svg { margin-right: 6px; display: block; }
                    }
                }
            }

            @media (max-width: 768px) {
                flex-direction: column;
                gap: 10px;
                padding: 14px 16px;

                span { font-size: 14px; }

                .sticky-buttons {
                    width: 100%;

                    a {
                        flex: 1;
                        text-align: center;
                    }
                }
            }
        }
    "##);
}
