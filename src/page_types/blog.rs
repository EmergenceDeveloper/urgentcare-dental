use crate::prelude::*;

pub fn construct_blog(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {


    let head = site.construct_head(page);

    let all_posts = get_all_posts();
    
    add_blog_index_schema(page, &all_posts);

    let mut post_list = Vec::new();

    for post in all_posts {
        let title = &post.frontmatter.title;
        let description = &post.frontmatter.description;
        let image = &post.frontmatter.image;
        let date = format_date(&post.frontmatter.date);
        
        // Get the post link from the slug
        let link = format!("/{}", post.slug);
        
        let category = post.frontmatter.category
            .first()
            .map(|c| c.as_str())
            .unwrap_or("Uncategorized");
        
        // Get up to three tags
        let tags_html = post.frontmatter.tags
            .iter()
            .take(3)
            .map(|tag| format!(r##"<span class="tag">{}</span>"##, tag))
            .collect::<Vec<_>>()
            .join("");
        
        let image_html = if !image.is_empty() {
            format!(r##"<img src="{image}" alt="{title}">"##)
        } else {
            String::new()
        };

        let post_item = format!(r##"
            <article class="post">
                <a href="{link}">
                    <div class="image-area">
                        {image_html}
                    </div>
                    <div class="text-area">
                        <div class="tags">
                            {tags_html}
                        </div>
                        <h3 class="title">{title}</h3>
                        <p>{description}</p>
                        <div class="date"><p>{date}</p></div>
                    </div>
                </a>
            </article>
    "##);

        post_list.push(post_item);
    }

    // Join all post items into a single string
    let posts_html = post_list.join("\n");
    
    let html = format!(r##"
        <!DOCTYPE html>
        
        <html lang="en-US">
        {head}
        <body>
            {header}
            <main class="blog">
                <section class="blog-intro">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="inner">
                        <div class="text-area">
                            <h1>Blog</h1>
                            <p>Your complete UK dental information hub. We publish the latest data, costs, and trends across Britain's dental services - from emergency wait times to treatment pricing across major cities.</p>
                        </div>
                    </div>
                
                </section>
                <section class="posts">
                    <div class="inner">
                        <div class="grid">
                            {posts}
                        </div>
                    </div>
                </section>
            </main>
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
        posts = posts_html,
    );
    
    
    
    
    
    
    
    site.declare_css("blog", r##"
    
        main.blog {
        
        
        
            /* Staggered animation */
            @keyframes fadeIn {
                from {
                    opacity: 0;
                    transform: translateY(20px);
                }
                to {
                    opacity: 1;
                    transform: translateY(0);
                }
            }
            
            
            section.blog-intro {
                
                display: block;
                position: relative;
                margin: 0 auto;
                
                
                
                .polka-dots {
                    position: absolute;
                    width: 100%;
                    height: 100%;
                    background-color: transparent;
                    background-image: radial-gradient(rgb(2, 220, 227) 1px, transparent 1px), radial-gradient(rgb(2, 220, 227) 1px, rgba(35, 84, 84, 0) 1px);
                    background-position: 0px 0px, 20px 20px;
                    background-size: 40px 40px;
                    border-radius: 0px;
                }
                
                .background-fade {
                    position: absolute;
                    z-index: 1;
                    width: 100%;
                    height: 100%;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background: linear-gradient(150deg, rgba(255, 255, 255, 0) 0%, rgb(255, 255, 255) 70%);
                }
                
            
                
                .inner {
                    
                    max-width: 1200px;
                    display: flex;
                    align-items: end;
                    padding-bottom: 48px;
                    padding: 200px var(--site-padding-x) 0;
                    margin: 0 auto;
                    
                    .text-area {
                    
                        margin: 0;
                        z-index: 10;
                    
                        .subtitle {
                            font-size: 16px;
                            color: var(--turquoise-30);
                            font-weight: 600;
                            margin-bottom: 24px;
                        }
                        
                        h1 {
                            font-size: 48px;
                            color: var(--turquoise-15);
                            margin-bottom: 24px;
                        }
                        
                        p {
                            font-size: 18px;
                            line-height: 1.6;
                            color: var(--grey-50);
                            margin: 0 auto 20px;
                            font-weight: 300;  
                            
                            &:last-child {
                                margin-bottom: 0;
                            }
                        }
                        
                        h2 {
                            font-size: 16px;
                            color: var(--turquoise-15);
                            margin-bottom: 24px;
                        }
       
                    
                    
                        
                    }

                    
                    
                }
                
            }
            
            
            section.posts {
            
                .inner {
                
                    display: grid;
                    grid-template-columns: (2, 1fr);
                    grid-gap: 24px;
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 0 var(--site-padding-x);
                    
                    .grid {
                    
                        display: grid;
                        grid-template-columns: repeat(2, 1fr);
                        grid-auto-flow: row;
                        gap: 2rem;
                        padding: 48px 0 120px;
                        
                        a, a:hover {
                            
                            text-decoration: none;
                            color: inherit;
        
                        }
                    
                        .post {

                            animation: fadeIn 0.5s ease-in-out;
                            
                            &:nth-child(odd) {
                                animation-delay: 0.1s;
                            }
                            
                            &:nth-child(even) {
                                animation-delay: 0.2s;
                            }
                            
                            .image-area {
                            
                                margin-bottom: 12px;
                            
                                img {
                                
                                    aspect-ratio: 2/1;
                                    object-fit: cover;
                                }
                                
                            }
                            
                            .text-area {
                            
                                display: flex;
                                flex-direction: column;
                                gap: 12px;
                                
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
                            
                                
                                h2, h3 {
                                    color: var(--turquoise-15);
                                    font-size: 18px;
                                    font-weight: 600;
                                    margin-bottom: 0;
                                }
                                
                                p {
                                    font-size: 14px;
                                    color: var(--grey-50);
                                    margin-bottom: 0;
                                
                                }
                            
                            
                            }
                        }
                        
                    
                    }
                    
                    

                    
                }
                
            
            }
            

        }
        
        
        
        
    
    "##);
    
    
    
    page.foundation.content = Some(html);
}