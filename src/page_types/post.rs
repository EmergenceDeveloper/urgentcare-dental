// In src/page_types/post.rs
use crate::prelude::*;

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
    let category = post.frontmatter.category
        .first()
        .map(|c| c.as_str())
        .unwrap_or("Uncategorized");

    let category_slug = slugify(category);
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
        
        format!(r##"
            <div class="meta">
                <div class="published">Published: {date}</div>
                {updated_html}
            </div>
        "##)
    };
    
    
    let author_card = format!(r##"
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
    "##);
    
    let tags_html = post.frontmatter.tags
        .iter()
        .take(3)
        .map(|tag| format!(r##"<span class="tag">{}</span>"##, tag))
        .collect::<Vec<_>>()
        .join("");
    
    let html = format!(r##"
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
                                <span class="separator">›</span>
                                <a href="/{category_slug}">{category}</a>
                            </div>
                            {meta_html}
                        </div>
                        {author_card}
                        <h1>{title}</h1>
                        <div class="main-image">
                            <img src="{image}" alt="{title}">
                        </div>
                        <div class="tags">
                            {tags_html}
                        </div>
                        <div class="content">
                            {content}
                        </div>
                    </article>
                </div>
            </main>
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
        title = post.frontmatter.title,
        meta_html = meta_html,
        image = post.frontmatter.image,
        content = post.content,
        category = category,
    );
    
    page.foundation.content = Some(html);
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
                    
                    .content {
                        font-size: 18px;
                        line-height: 1.625;
                        color: rgb(35, 35, 35);
                        letter-spacing: -0.18px
                        
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
                            display: block;
                            
                            tr {
                                background-color: #fffefc;
                                word-break: normal;
                                
                                &:nth-child(even) {
                                    background-color: #fcfce3;
                                }
                            }
                            
                            tbody:only-child {
                                tr {
                                    &:nth-child(odd) {
                                        background-color: #fcfce3;
                                    }
                                    
                                    &:nth-child(even) {
                                        background-color: #fffefc;
                                    }
                                    
                                    &:first-child {
                                        background-color: #f2ec2c;
                                        font-weight: 600;
                                        color: #cd7f32;
                                        
                                        td {
                                            font-weight: 600;
                                        }
                                    }
                                }
                            }
                            
                            th {
                                background-color: #f2ec2c;
                                font-weight: 600;
                                color: #cd7f32;
                                text-align: left;
                                padding: 12px;
                            }
                            
                            td {
                                padding: 12px;
                                border: 1px solid #e3d4af;
                                text-align: left;
                                vertical-align: middle;
                            }
                            
                            th {
                                border: 1px solid #e3d4af;
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
    "##);
}