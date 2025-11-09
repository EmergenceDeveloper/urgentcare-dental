use crate::prelude::*;
use serde_json::json;

pub fn add_sitewide_schema<T, I>(site: &mut Site<T, I>) {
    let website_schema = json!({
        "@context": "https://schema.org",
        "@type": "WebSite",
        "@id": format!("{}#website", SITE_URL),
        "url": SITE_URL,
        "name": SITE_NAME,
        "description": SITE_DESCRIPTION,
        "publisher": {"@id": format!("{}#organization", SITE_URL)},
        "inLanguage": "en-GB"
    });
    
    let organization_schema = json!({
        "@context": "https://schema.org",
        "@type": ["Organization", "Dentist", "MedicalOrganization"],
        "@id": format!("{}#organization", SITE_URL),
        "name": SITE_NAME,
        "url": SITE_URL,
        "telephone": PHONE_NUMBER,
        "priceRange": "££",
        "address": {
            "@type": "PostalAddress",
            "streetAddress": "313a Roundhay Rd Harehills",
            "addressLocality": "Leeds",
            "addressRegion": "West Yorkshire",
            "postalCode": "LS8 4HT",
            "addressCountry": "GB"
        },
        "geo": {
            "@type": "GeoCoordinates",
            "latitude": "53.81877457313221",
            "longitude": "-1.5152765134916233"
        },
        "areaServed": [
            {
                "@type": "City",
                "name": "Leeds"
            },
            {
                "@type": "City",
                "name": "Manchester"
            },
            {
                "@type": "AdministrativeArea",
                "name": "Northern England"
            }
        ],
        "openingHoursSpecification": [
            {
                "@type": "OpeningHoursSpecification",
                "dayOfWeek": [
                    "Monday",
                    "Tuesday",
                    "Wednesday",
                    "Thursday",
                    "Friday",
                    "Saturday",
                    "Sunday"
                ],
                "opens": "00:00",
                "closes": "23:59"
            }
        ],
        "aggregateRating": {
            "@type": "AggregateRating",
            "ratingValue": "4.9",
            "reviewCount": "292",
            "bestRating": "5",
            "worstRating": "1"
        },
        "review": [
            {
                "@type": "Review",
                "author": {
                    "@type": "Person",
                    "name": "Sarah Mitchell"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5",
                    "bestRating": "5"
                },
                "reviewBody": "Sorted my broken crown same evening. Clean modern practice, caring staff. Called several dentists but only these could help after hours. Quick response on the phone and clear pricing upfront. The dentist explained everything thoroughly."
            },
            {
                "@type": "Review",
                "author": {
                    "@type": "Person",
                    "name": "Emma Wilson"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5",
                    "bestRating": "5"
                },
                "reviewBody": "Relief from abscess pain when my regular dentist was closed. Worth every penny for evening care. Was worried about needing emergency treatment but they made the whole process straightforward. Excellent follow-up care instructions too."
            },
            {
                "@type": "Review",
                "author": {
                    "@type": "Person",
                    "name": "Jennifer O'Connor"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5",
                    "bestRating": "5"
                },
                "reviewBody": "Emergency visit for broken crown. Reception was efficient and got me in within 30 minutes. Thorough treatment and helpful aftercare advice. Great to know they're available when needed."
            },
            {
                "@type": "Review",
                "author": {
                    "@type": "Person",
                    "name": "David Thompson"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5",
                    "bestRating": "5"
                },
                "reviewBody": "Sorted my broken crown same evening. Clean modern practice, caring staff. Called several dentists but only these could help after hours. Quick response on the phone and clear pricing upfront. The dentist explained everything thoroughly."
            },
            {
                "@type": "Review",
                "author": {
                    "@type": "Person",
                    "name": "David Chen"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5",
                    "bestRating": "5"
                },
                "reviewBody": "Sudden toothache after work - really relieved they could see me same evening. Quick appointment and professional service. Dentist explained everything clearly and fixed the problem. Clean, modern clinic."
            },
            {
                "@type": "Review",
                "author": {
                    "@type": "Person",
                    "name": "Fatima Rahman"
                },
                "reviewRating": {
                    "@type": "Rating",
                    "ratingValue": "5",
                    "bestRating": "5"
                },
                "reviewBody": "Called with severe pain from abscess. Professional team and prompt service got me sorted quickly. Clean facilities and caring staff made a stressful situation much easier. Very grateful for their help."
            }
        ],
        "hasOfferCatalog": {
            "@type": "OfferCatalog",
            "name": "Emergency & Dental Services",
            "itemListElement": [
                {
                    "@type": "Offer",
                    "itemOffered": {
                        "@type": "MedicalProcedure",
                        "name": "Emergency Dental Appointment",
                        "description": "Immediate attention for urgent dental issues, including pain relief"
                    },
                    "price": "20",
                    "priceCurrency": "GBP",
                    "availability": "https://schema.org/InStock"
                },
                {
                    "@type": "Offer",
                    "itemOffered": {
                        "@type": "MedicalProcedure",
                        "name": "Simple Tooth Extraction",
                        "description": "Removal of damaged or impacted teeth"
                    },
                    "price": "149",
                    "priceCurrency": "GBP",
                    "availability": "https://schema.org/InStock"
                },
                {
                    "@type": "Offer",
                    "itemOffered": {
                        "@type": "MedicalProcedure",
                        "name": "Complex Tooth Extraction",
                        "description": "Removal of damaged or impacted teeth requiring specialized techniques"
                    },
                    "price": "349",
                    "priceCurrency": "GBP",
                    "availability": "https://schema.org/InStock"
                },
                {
                    "@type": "Offer",
                    "itemOffered": {
                        "@type": "MedicalProcedure",
                        "name": "Surgical Tooth Extraction",
                        "description": "Advanced surgical removal of impacted or difficult teeth"
                    },
                    "price": "549",
                    "priceCurrency": "GBP",
                    "availability": "https://schema.org/InStock"
                },
                {
                    "@type": "Offer",
                    "itemOffered": {
                        "@type": "MedicalProcedure",
                        "name": "Dental Filling",
                        "description": "Restoration of cavities to prevent further tooth decay"
                    },
                    "price": "99",
                    "priceCurrency": "GBP",
                    "availability": "https://schema.org/InStock"
                },
                {
                    "@type": "Offer",
                    "itemOffered": {
                        "@type": "MedicalProcedure",
                        "name": "Dental Implant",
                        "description": "Permanent tooth replacement using surgical implants"
                    },
                    "price": "1999",
                    "priceCurrency": "GBP",
                    "availability": "https://schema.org/InStock"
                }
            ]
        },
        "paymentAccepted": ["Cash", "Credit Card", "Debit Card", "Bank Transfer"],
        "currenciesAccepted": "GBP",
        "availableLanguage": "en-GB",
        "medicalSpecialty": "Emergency Dentistry",
        "serviceType": "Emergency Dental Care",
        "serviceArea": {
            "@type": "GeoCircle",
            "geoMidpoint": {
                "@type": "GeoCoordinates",
                "latitude": "53.81877457313221",
                "longitude": "-1.5152765134916233"
            },
            "geoRadius": "50000"
        }
    });
    
    let weblog_schema = json!({
        "@context": "https://schema.org",
        "@type": "Blog",
        "name": SITE_NAME,
        "url": SITE_URL,
        "description": SITE_DESCRIPTION,
        "publisher": {
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL)
            }
        }
    });
    
    let graph = json!({
        "@context": "https://schema.org",
        "@graph": [website_schema, organization_schema, weblog_schema]
    });
    
    let schema_script = format!(
        r##"<script type="application/ld+json">{}</script>"##,
        graph.to_string()
    );
    
    site.declare_placement(PlacementPosition::Schema, &schema_script);
}

pub fn add_blog_post_schema(
    page: &mut Page<UCDPages>,
    post: &PostData<UCDFrontmatter>
) {
    // Format dates to ISO 8601 - handle dates that may already have T in them
    let date_published = if !post.frontmatter.date.is_empty() {
        if post.frontmatter.date.contains('T') {
            post.frontmatter.date.clone()
        } else {
            format!("{}T00:00:00+00:00", post.frontmatter.date)
        }
    } else {
        String::new()
    };
    
    let date_modified = if !post.frontmatter.updated.is_empty() {
        if post.frontmatter.updated.contains('T') {
            post.frontmatter.updated.clone()
        } else {
            format!("{}T00:00:00+00:00", post.frontmatter.updated)
        }
    } else if !date_published.is_empty() {
        date_published.clone()
    } else {
        String::new()
    };
    
    // Use author if set, otherwise use UCD Editorial Team
    let (author, author_id) = if !post.frontmatter.author.is_empty() {
        (
            json!({
                "@type": "Person",
                "name": post.frontmatter.author
            }),
            format!("https://urgentcaredental.co.uk/author/{}#person", slugify(&post.frontmatter.author))
        )
    } else {
        (
            json!({
                "@type": "Person",
                "@id": "https://urgentcaredental.co.uk/author/editorial-team#person",
                "name": "UCD Editorial Team",
                "jobTitle": "Department of Dentistry Journalism",
                "worksFor": {"@id": format!("{}#organization", SITE_URL)}
            }),
            "https://urgentcaredental.co.uk/author/editorial-team#person".to_owned()
        )
    };
    
    // Build image array for better SEO
    let image = if !post.frontmatter.image.is_empty() {
        json!([
            post.frontmatter.image.clone(),
            {
                "@type": "ImageObject",
                "url": post.frontmatter.image.clone(),
                "width": 1200,
                "height": 630
            }
        ])
    } else {
        json!([format!("{}images/logo.png", SITE_URL)])
    };
    
    let article_url = format!("{}{}", SITE_URL, post.slug);
    
    let article_schema = json!({
        "@context": "https://schema.org",
        "@type": "Article",
        "@id": format!("{}#article", article_url),
        "isPartOf": {"@id": article_url},
        "author": {
            "name": if !post.frontmatter.author.is_empty() { 
                post.frontmatter.author.clone() 
            } else { 
                "UCD Editorial Team".to_owned() 
            },
            "@id": author_id,
            "jobTitle": if post.frontmatter.author.is_empty() { 
                Some("Department of Dentistry Journalism") 
            } else { 
                None 
            },
            "worksFor": if post.frontmatter.author.is_empty() { 
                Some(json!({"@id": format!("{}#organization", SITE_URL)})) 
            } else { 
                None 
            }
        },
        "headline": post.frontmatter.title.clone(),
        "description": post.frontmatter.description.clone(),
        "image": image,
        "datePublished": date_published,
        "dateModified": date_modified,
        "mainEntityOfPage": {"@id": article_url},
        "publisher": {"@id": format!("{}#organization", SITE_URL)},
        "articleSection": post.frontmatter.category.first().map(|c| vec![c.clone()]).unwrap_or_else(Vec::new),
        "keywords": post.frontmatter.tags.join(", "),
        "wordCount": post.content.split_whitespace().count(),
        "inLanguage": "en-GB"
    });
    
    let webpage_schema = json!({
        "@context": "https://schema.org",
        "@type": "WebPage",
        "@id": article_url,
        "url": article_url,
        "name": post.frontmatter.title.clone(),
        "isPartOf": {"@id": format!("{}#website", SITE_URL)},
        "datePublished": date_published,
        "dateModified": date_modified,
        "inLanguage": "en-GB"
    });
    
    // Add breadcrumb schema
    let category = post.frontmatter.category
        .first()
        .map(|c| c.as_str())
        .unwrap_or("Uncategorized");
    
    let breadcrumb_schema = json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {
                "@type": "ListItem",
                "position": 1,
                "name": "Home",
                "item": SITE_URL
            },
            {
                "@type": "ListItem",
                "position": 2,
                "name": "Blog",
                "item": format!("{}blog", SITE_URL)
            },
            {
                "@type": "ListItem",
                "position": 3,
                "name": category,
                "item": format!("{}{}", SITE_URL, slugify(category))
            },
            {
                "@type": "ListItem",
                "position": 4,
                "name": post.frontmatter.title.clone()
            }
        ]
    });
    
    // Add FAQ schema if the content has headers that look like questions
    let faq_items = extract_faq_from_content(&post.content);
    let faq_schema = if !faq_items.is_empty() {
        Some(json!({
            "@context": "https://schema.org",
            "@type": "FAQPage",
            "mainEntity": faq_items
        }))
    } else {
        None
    };
    
    let mut graph_items = vec![article_schema, webpage_schema, author, breadcrumb_schema];
    if let Some(faq) = faq_schema {
        graph_items.push(faq);
    }
    
    let graph = json!({
        "@context": "https://schema.org",
        "@graph": graph_items
    });
    
    let schema_script = format!(
        r##"<script type="application/ld+json">{}</script>"##,
        graph.to_string()
    );
    
    page.declare_placement(PlacementPosition::Schema, &schema_script);
}

pub fn add_blog_index_schema(
    page: &mut Page<UCDPages>,
    posts: &[PostData<UCDFrontmatter>]
) {
    // Build item list of all blog posts
    let post_items: Vec<serde_json::Value> = posts.iter().enumerate().map(|(index, post)| {
        let date_published = if !post.frontmatter.date.is_empty() {
            if post.frontmatter.date.contains('T') {
                post.frontmatter.date.clone()
            } else {
                format!("{}T00:00:00+00:00", post.frontmatter.date)
            }
        } else {
            String::new()
        };
        
        json!({
            "@type": "ListItem",
            "position": index + 1,
            "item": {
                "@type": "BlogPosting",
                "headline": post.frontmatter.title.clone(),
                "url": format!("{}{}", SITE_URL, post.slug),
                "description": post.frontmatter.description.clone(),
                "datePublished": date_published,
                "author": if !post.frontmatter.author.is_empty() {
                    json!({
                        "@type": "Person",
                        "name": post.frontmatter.author.clone()
                    })
                } else {
                    json!({
                        "@type": "Organization",
                        "name": SITE_NAME
                    })
                }
            }
        })
    }).collect();
    
    let collection_schema = json!({
        "@context": "https://schema.org",
        "@type": "CollectionPage",
        "name": format!("{} Blog", SITE_NAME),
        "description": SITE_DESCRIPTION,
        "url": format!("{}blog", SITE_URL),
        "isPartOf": {
            "@type": "Blog",
            "name": SITE_NAME,
            "url": SITE_URL
        },
        "publisher": {
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL)
            }
        },
        "numberOfItems": posts.len(),
        "mainEntity": {
            "@type": "ItemList",
            "itemListElement": post_items
        },
        "inLanguage": "en-US"
    });
    
    let breadcrumb_schema = json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {
                "@type": "ListItem",
                "position": 1,
                "name": "Home",
                "item": SITE_URL
            },
            {
                "@type": "ListItem",
                "position": 2,
                "name": "Blog"
            }
        ]
    });
    
    let graph = json!({
        "@context": "https://schema.org",
        "@graph": [collection_schema, breadcrumb_schema]
    });
    
    let schema_script = format!(
        r##"<script type="application/ld+json">{}</script>"##,
        graph.to_string()
    );
    
    page.declare_placement(PlacementPosition::Schema, &schema_script);
}

pub fn add_category_schema(
    page: &mut Page<UCDPages>,
    category_name: &str,
    posts: &[PostData<UCDFrontmatter>]
) {
    // Build item list of all posts in this category
    let post_items: Vec<serde_json::Value> = posts.iter().enumerate().map(|(index, post)| {
        let date_published = if !post.frontmatter.date.is_empty() {
            if post.frontmatter.date.contains('T') {
                post.frontmatter.date.clone()
            } else {
                format!("{}T00:00:00+00:00", post.frontmatter.date)
            }
        } else {
            String::new()
        };
        
        json!({
            "@type": "ListItem",
            "position": index + 1,
            "item": {
                "@type": "BlogPosting",
                "headline": post.frontmatter.title.clone(),
                "url": format!("{}{}", SITE_URL, post.slug),
                "description": post.frontmatter.description.clone(),
                "datePublished": date_published
            }
        })
    }).collect();
    
    let collection_schema = json!({
        "@context": "https://schema.org",
        "@type": "CollectionPage",
        "name": format!("{} - {}", category_name, SITE_NAME),
        "description": format!("Browse all {} posts in the {} category on {}", posts.len(), category_name, SITE_NAME),
        "url": format!("{}{}", SITE_URL, slugify(category_name)),
        "isPartOf": {
            "@type": "Blog",
            "name": SITE_NAME,
            "url": SITE_URL
        },
        "publisher": {
            "@type": "Organization",
            "name": SITE_NAME,
            "logo": {
                "@type": "ImageObject",
                "url": format!("{}images/logo.png", SITE_URL)
            }
        },
        "numberOfItems": posts.len(),
        "mainEntity": {
            "@type": "ItemList",
            "itemListElement": post_items
        },
        "inLanguage": "en-US"
    });
    
    let breadcrumb_schema = json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {
                "@type": "ListItem",
                "position": 1,
                "name": "Home",
                "item": SITE_URL
            },
            {
                "@type": "ListItem",
                "position": 2,
                "name": "Blog",
                "item": format!("{}blog", SITE_URL)
            },
            {
                "@type": "ListItem",
                "position": 3,
                "name": category_name
            }
        ]
    });
    
    let graph = json!({
        "@context": "https://schema.org",
        "@graph": [collection_schema, breadcrumb_schema]
    });
    
    let schema_script = format!(
        r##"<script type="application/ld+json">{}</script>"##,
        graph.to_string()
    );
    
    page.declare_placement(PlacementPosition::Schema, &schema_script);
}

// Helper function to extract FAQ items from content
fn extract_faq_from_content(content: &str) -> Vec<serde_json::Value> {
    let mut faq_items = Vec::new();
    
    // Look for headers that end with "?" as potential FAQ questions
    // This is a simple implementation - you could make it more sophisticated
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Check if it's a header with a question
        if (line.starts_with("## ") || line.starts_with("### ")) && line.ends_with("?") {
            let question = line.trim_start_matches('#').trim();
            
            // Collect answer until next header or end
            let mut answer = String::new();
            i += 1;
            
            while i < lines.len() {
                let next_line = lines[i];
                if next_line.starts_with("#") {
                    break;
                }
                if !next_line.trim().is_empty() {
                    if !answer.is_empty() {
                        answer.push(' ');
                    }
                    answer.push_str(next_line.trim());
                }
                i += 1;
            }
            
            if !answer.is_empty() {
                faq_items.push(json!({
                    "@type": "Question",
                    "name": question,
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": answer
                    }
                }));
            }
        } else {
            i += 1;
        }
    }
    
    faq_items
}