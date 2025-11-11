use urgentcare_dental::prelude::*;
use std::collections::HashMap;
use UCDPages::*;

fn main() {
    
    let mut pages = vec![
        Page {
            foundation: PageFoundation { 
                title: "UrgentCare Dental | £20 Visit (+ Free X-rays)".to_owned(),
                slug: Some("/".to_owned()),
                metadescription: Some("This is your 24/7 emergency dentist in Northern England. Learn how to get immediate dental pain relief with our £20 assessment and same-day treatment at our Leeds and Manchester clinics.".to_owned()),
                ..default() 
            },
            specification: Homepage,
        },
        Page {
            foundation: PageFoundation { 
                title: "Blog".to_owned(),
                slug: Some("blog".to_owned()),
                metadescription: Some("This is your complete UK dental information hub. Learn facts, data, and trends about dental care across Britain.".to_owned()),
                ..default() 
            },
            specification: PostList,
        },
        /*Page {
            foundation: PageFoundation { 
                title: "Categories".to_owned(),
                slug: Some("categories".to_owned()),
                metadescription: Some("Browse all content categories".to_owned()),
                ..default() 
            },
            specification: CategoriesIndex,
        },*/
        Page {
            foundation: PageFoundation { 
                title: "About Us".to_owned(),
                slug: Some("about-us".to_owned()),
                metadescription: Some("This is your trusted emergency dental team in Northern England. Learn how our experienced dentists provide 24/7 urgent care and same-day treatment in our Leeds and Manchester clinics.".to_owned()),
                ..default() 
            },
            specification: AboutUs,
        },
        Page {
            foundation: PageFoundation { 
                title: "Contact".to_owned(),
                slug: Some("contact".to_owned()),
                metadescription: Some("This is your direct line to emergency dental care. Learn how to reach our 24/7 team in Leeds and Manchester by phone, WhatsApp, or online booking from £20.".to_owned()),
                ..default() 
            },
            specification: Contact,
        },
        Page {
            foundation: PageFoundation { 
                title: "Pricing".to_owned(),
                slug: Some("pricing".to_owned()),
                metadescription: Some("This is your complete emergency dental pricing guide. Learn how our transparent fees start from £20 for pain relief, with clear costs for all urgent and routine treatments across our clinics.".to_owned()),
                ..default() 
            },
            specification: Pricing,
        },
        Page {
            foundation: PageFoundation { 
                title: "Dental Implants That Last a Lifetime | Free Consultations".to_owned(),
                slug: Some("implants".to_owned()),
                metadescription: Some("This is our dental implants page. Learn more about our implants, which are the highest quality tooth replacement solution modern dentistry has to offer. Free consultations.".to_owned()),
                ..default() 
            },
            specification: Implants,
        },
        Page {
            foundation: PageFoundation { 
                title: "Privacy Policy".to_owned(),
                slug: Some("privacy-policy".to_owned()),
                metadescription: Some("This is UrgentCare Dental's privacy policy. Learn how we collect, protect, and use your personal information when providing emergency dental care services.".to_owned()),
                ..default() 
            },
            specification: PolicyPage,
        },
        Page {
            foundation: PageFoundation { 
                title: "Terms and Conditions".to_owned(),
                slug: Some("terms-and-conditions".to_owned()),
                metadescription: Some("This is UrgentCare Dental's terms and conditions. Learn about our terms of service and policies here.".to_owned()),
                ..default() 
            },
            specification: PolicyPage,
        },
        Page {
            foundation: PageFoundation { 
                title: "Complaints Procedure".to_owned(),
                slug: Some("complaints".to_owned()),
                metadescription: Some("This is UrgentCare Dental's complaints procedure. Learn how we resolve your complaints, and how to contact us.".to_owned()),
                ..default() 
            },
            specification: PolicyPage,
        },
        Page {
            foundation: PageFoundation { 
                title: "Cookie Policy".to_owned(),
                slug: Some("cookie-policy".to_owned()),
                metadescription: Some("This is UrgentCare Dental's cookie policy. Learn how we use cookies and similar technologies to improve your emergency dental care experience.".to_owned()),
                ..default() 
            },
            specification: PolicyPage,
        },
        Page {
            foundation: PageFoundation { 
                title: "Dental Membership Plans".to_owned(),
                slug: Some("plans".to_owned()),
                metadescription: Some("These are UrgentCare Dental's membership plans. Learn how you can save on your dental treatment costs with our Protection Plan and Proactive Recovery Plan.".to_owned()),
                ..default() 
            },
            specification: Plans,
        },

    ];
    
    // Collect blog posts
    /*for post in get_all_posts_and_post_pages() {
        pages.push(Page {
            foundation: PageFoundation {
                title: post.frontmatter.title.clone(),
                slug: Some(post.slug.clone()),
                metadescription: Some(post.frontmatter.description.clone()),
                ..default()
            },
            specification: UCDPages::BlogPost(post),
        });
    }*/
    
    // Get all posts
    let all_posts = get_all_posts_and_post_pages();
    
    // Build a map of categories to their posts
    /*let mut categories: HashMap<String, Vec<PostData<UCDFrontmatter>>> = HashMap::new();*/
    
    // Add blog posts and collect categories
    for post in &all_posts {
        // Add the blog post page
        pages.push(Page {
            foundation: PageFoundation {
                title: post.frontmatter.title.clone(),
                slug: Some(post.slug.clone()),
                metadescription: Some(post.frontmatter.description.clone()),
                ..default()
            },
            specification: UCDPages::BlogPost(post.clone()),
        });
        
        // Collect posts by category
        /*if !post.frontmatter.exclude {  // Only include non-excluded posts in categories
            if post.frontmatter.category.is_empty() {
                // Add to Uncategorized if no categories
                categories
                    .entry("Uncategorized".to_owned())
                    .or_insert_with(Vec::new)
                    .push(post.clone());
            } else {
                // Add to each specified category
                for category in &post.frontmatter.category {
                    categories
                        .entry(category.clone())
                        .or_insert_with(Vec::new)
                        .push(post.clone());
                }
            }
        }*/
    }
    
    // Create category pages
    /*for (category_name, mut category_posts) in categories {
        // Sort posts by date (newest first) within each category
        category_posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
        
        let category_slug = slugify(&category_name);
        
        pages.push(Page {
            foundation: PageFoundation {
                title: category_name.to_owned(),
                slug: Some(category_slug),
                metadescription: Some(format!("Browse all {} posts in the {} category", 
                    category_posts.len(), category_name)),
                ..default()
            },
            specification: UCDPages::Category(category_name, category_posts),
        });
    }*/
    
    let mut site = Site::<UCDPages, ()> {
        title: SITE_NAME.to_owned(),
        base_url: Url::parse(SITE_URL).unwrap(),
        settings: Settings {
            use_trailing_slashes: false,
            title_append: Some(" - UrgentCare Dental".to_owned()),
            
            ..default()
        },
        ..default()  // No imperium needed for this simple site
    }
    .add_constructor(Homepage, construct_homepage)
    .add_constructor(BlogPost(default()), construct_post)
    .add_constructor(PostList, construct_blog)
    /*.add_constructor(Category(String::new(), Vec::new()), construct_category)
    .add_constructor(CategoriesIndex, construct_categories)*/
    .add_constructor(AboutUs, construct_about_us)
    .add_constructor(Contact, construct_contact)
    .add_constructor(Pricing, construct_pricing)
    .add_constructor(Implants, construct_implants)
    .add_constructor(PolicyPage, construct_policy_pages)
    .add_constructor(Plans, construct_plans)
    // Your constructors will go here
    .add_head_constructor()
    .add_pages(pages);
    
    
    add_core_css(&mut site);
    
    add_decrees(&mut site);
    
    add_head_code(&mut site);
    
    add_sitewide_schema(&mut site);
    
    add_common_sections(&mut site);
    
    
    site.commence();
}