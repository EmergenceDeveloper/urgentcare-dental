use crate::prelude::*;

pub fn construct_homepage(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    add_default_og_image(page);
    let head = site.construct_head(page);
    
    
    css(site);
    
    
    let header = construct_header(site, &page.foundation);
    let footer = construct_footer(site);
    
    let dentist_splash_hero = &site.sections["dentist_splash_hero"];
    let about_our_clinics = &site.sections["about_our_clinics"];
    let locations = &site.sections["locations"];
    let services = &site.sections["services"];
    let general_services = &site.sections["general_services"];
    let meet_us = &site.sections["meet_us"];
    let open_times = &site.sections["open_times"];
    let patient_stories = &site.sections["patient_stories"];
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-GB">
        {head}
        <body>
            {header}
            <main class="home">

                {dentist_splash_hero}
                {about_our_clinics}
                {locations}
                {services}
                {general_services}
                {meet_us}
                {open_times}
                {patient_stories}
                
            </main>
            {footer}
        </body>
        </html>
    "##
    );
    
    
    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("homepage", r##"
        {}
        
        main.home {
            
            
            .small-button {
                display: block;
                width: fit-content;
                font-size: 16px;
                font-weight: 500;
                height: min-content;
                place-content: center flex-start;
                align-items: center;
                /*padding: 0 12px;*/
                height: auto;
                text-decoration: none;
                align-self: center;
                place-content: center;
                background-color: #fff;
                color: var(--turquoise-15);
                
                padding: 14px 32px;
                border-radius: 25px;
                text-decoration: none;
                transition: all 0.3s ease;
                
                &.secondary {
                    background: white;
                    color: var(--turquoise-30);
                    border: 2px solid var(--turquoise-30);
                    
                    &:hover {
                        background: var(--turquoise-98);
                        transform: translateY(-2px);
                    }
                }

            }
        

            
        }
    "##);
}