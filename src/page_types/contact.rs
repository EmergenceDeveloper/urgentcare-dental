use crate::prelude::*;

pub fn construct_contact(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    let head = site.construct_head(page);
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-US">
        {head}
        <body>
            {header}
            <main class="contact">
            
            
                <section class="get-in-touch">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="inner">
                        <div class="text-area">
                            <h1>Get in Touch</h1>
                            <p>Get in touch with us with our contact details below. Our team will respond to your inquiries promptly.</p>    
                            <h2>Contact</h2>
                            <div class="contact-methods">
                                <a href="{EMAIL_LINK}">{EMAIL}</a>
                                <a href="{PHONE_NUMBER_LINK}">{PHONE_NUMBER}</a>
                            </div>
                        </div>
                    </div>
                
                </section>
                
                
                
                
                {open_times}
                
                {locations}
                
                {meet_us}
                
                
                
                
            </main>
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
        open_times = &site.sections["open_times"],
        locations = &site.sections["locations"],
        meet_us = &site.sections["meet_us"],
        
    );
    
    css(site);
    
    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("contact", r##"
    
    {}
        
        main.contact {
            
            
            
            section.get-in-touch {
                
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
                        max-width: 500px;
                        width: 55%;
                        z-index: 10;
                        
                        @media (max-width: 768px) {
                            width: 100%;
                        }
                    
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
                            
                            @media (max-width: 768px) {
                                font-size: 32px;
                            }
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
                        
                        .contact-methods {
                        
                            a {
                                display: block;
                                text-decoration: none;
                                
                                &:hover {
                                    text-decoration: underline;
                                }

                            }
                        
                        }
                    
                    
                        
                    }

                    
                    
                }
                
            }
            
            section.open-times {
                background-color: white;
                display: block;
                
                .inner {
                

                    .background-fade {
                        background: linear-gradient(330deg, rgba(2, 145, 150, 0) 0.1%, #ffffff, rgb(255 255 255) 140%);
                    }
                    
                    .text-area {
                        
                        
                        
                        h2 {
                            color: rgb(1, 73, 75);
                        }
                        
                        p {
                            color: var(--grey-50)  ;
                        }
                        
                    }
                }
            }
        }    
        
    "##);
}