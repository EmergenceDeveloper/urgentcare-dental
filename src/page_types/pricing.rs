use crate::prelude::*;

pub fn construct_pricing(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    add_default_og_image(page);
    let head = site.construct_head(page);
    
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-GB">
        {head}
        <body>
            {header}
            <main class="pricing">
            
            
                <section class="get-in-touch">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="inner">
                        <div class="text-area">
                            <h1>Transparent Pricing for Exceptional Dental Care</h1>
                            <p>Clear, upfront costs for every common dental procedure we offer. As an emergency and urgent care clinic, we understand you need to know exactly what you're paying for - especially during stressful situations.</p><p>All treatment prices below are guidelines for how much a simple case would cost, but we'll be able to get you a quote for your own treatment when we speak to you.</p>
    
                        </div>
                    </div>
                
                </section>
                
                <section class="pricing-tables">
                    <div class="inner">
                    
                        <h2>Specialized & Preventive Care</h2>
                        
                        <h3>Restorative Dentistry</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Dental Fillings</p>
                                <p>Restoration of cavities to prevent further tooth decay.</p>
                            </td>
                            <td>£99-£250 / filling</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Tooth Reconstructions</p>
                                <p>Reconstruction of teeth using tooth-colored composite material.</p>
                            </td>
                            <td>£500 / tooth</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Dental Crowns</p>
                                <p>Custom-crafted crowns to restore and strengthen damaged teeth.</p>
                            </td>
                            <td>£650 / crown</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Dental Bridges</p>
                                <p>Replacement of missing teeth with a fixed dental bridge.</p>
                            </td>
                            <td>£595 / unit</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Dental Implants</p>
                                <p>Permanent tooth replacement using surgical implants.</p>
                            </td>
                            <td>£1,999 / implant</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Dentures</p>
                                <p>Full or partial dentures for functional and aesthetic restoration.</p>
                            </td>
                            <td>£699 / arch</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Oral Surgery</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Simple Tooth Extractions</p>
                                <p>Removal of damaged or impacted teeth.</p>
                            </td>
                            <td>£149 / extraction</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Complex Tooth Extractions</p>
                                <p>Removal of damaged or impacted teeth.</p>
                            </td>
                            <td>£349 / extraction</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Surgical Extractions</p>
                                <p>Corrective surgical procedures to address jaw abnormalities.</p>
                            </td>
                            <td>£549 / extraction</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Wisdom Teeth Removal</p>
                                <p>Corrective surgical procedures to address jaw abnormalities.</p>
                            </td>
                            <td>£549 / tooth</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Jaw Surgery (Orthognathic Surgery)</p>
                                <p>Corrective surgical procedures to address jaw abnormalities.</p>
                            </td>
                            <td>Personalized quote</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Surgical Exposure of Impacted Canine</p>
                                <p>Surgical procedure to expose buried canine teeth for orthodontic treatment.</p>
                            </td>
                            <td>£699</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Frenectomy</p>
                                <p>Surgical correction of restrictive tissue bands affecting tongue or lip movement.</p>
                            </td>
                            <td>£299</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Emergency Dentistry</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Emergency Appointments</p>
                                <p>Immediate attention for urgent dental issues, including pain relief.</p>
                            </td>
                            <td>£20</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Follow-ups After Treatment</p>
                                <p>Follow-ups are available free of charge after treatment.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Traditional Braces</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Braces Consultation</p>
                                <p>Evaluation and discussion of orthodontic treatment options.</p>
                            </td>
                            <td>£20</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Traditional Metal Braces</p>
                                <p>Application and ongoing adjustments.</p>
                            </td>
                            <td>Personalized quote</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Clear Aligners</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Consultation</p>
                                <p>Evaluation and discussion of the treatment plan.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Clear Aligners</p>
                                <p>Customized series of clear aligners for discreet orthodontic treatment.</p>
                            </td>
                            <td>£2,999</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Retainers and Post-Treatment Care</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Follow-ups After Treatment</p>
                                <p>Follow-ups are available free of charge after treatment.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Fixed Retainers</p>
                                <p>Fixed retainers to maintain orthodontic results.</p>
                            </td>
                            <td>£199 / set</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Removable Retainers</p>
                                <p>Custom-made retainers to maintain orthodontic results.</p>
                            </td>
                            <td>£250 / set</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Sedation Dentistry</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>IV Sedation</p>
                                <p>Intravenous sedation for deeper relaxation during extensive procedures.</p>
                            </td>
                            <td>£300 - £500 / session</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>TMJ/TMD Treatment</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Diagnostic Consultation</p>
                                <p>Evaluation and discussion of TMJ/TMD symptoms.</p>
                            </td>
                            <td>£100</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Customized Splints and Mouthguards</p>
                                <p>Oral appliances to alleviate TMJ/TMD symptoms.</p>
                            </td>
                            <td>Personalized quote</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Lesion Removal</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Intra-Oral Lesion Removal</p>
                                <p>Professional removal of papilloma, mucocele, fibro-epithelial polyp, and salivary gland stones inside the mouth.</p>
                            </td>
                            <td>£499</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Extra-Oral Lesion Removal</p>
                                <p>Professional removal of skin warts, sebaceous cysts, sun damage, Tugse, and Botox-related complications on facial areas.</p>
                            </td>
                            <td>£499</td>
                            </tr>
                        </tbody>
                        </table>
                        
                        <h2>General & Cosmetic Dentistry Prices</h2>
                        
                        <h3>Comprehensive Dental Exams</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Initial Consultation</p>
                                <p>Comprehensive evaluation and discussion of oral health concerns.</p>
                            </td>
                            <td>£20</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Digital X-rays</p>
                                <p>Advanced digital imaging for detailed diagnostics.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Oral Cancer Screening</p>
                                <p>Specialized screening to detect early signs of oral cancer.</p>
                            </td>
                            <td>£50</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Periodontal Assessment</p>
                                <p>Evaluation of gum health and identification of potential issues.</p>
                            </td>
                            <td>£80</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Follow-ups After Treatment</p>
                                <p>Follow-ups are available free of charge after treatment.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Professional Teeth Cleaning</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Standard Cleaning (Prophylaxis)</p>
                                <p>Removal of plaque and tartar for a clean and refreshed feeling.</p>
                            </td>
                            <td>£100</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Deep Cleaning (Scaling and Root Planing)</p>
                                <p>Intensive cleaning for patients with advanced gum disease.</p>
                            </td>
                            <td>£150 - £180</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Periodontal Maintenance</p>
                                <p>Ongoing cleaning and care for patients with gum disease.</p>
                            </td>
                            <td>£120</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Digital X-rays and Diagnostics</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Diagnostic X-rays</p>
                                <p>Professional teeth whitening performed in the clinic.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Panoramic X-ray</p>
                                <p>Comprehensive imaging for a broader view of oral structures.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                            <tr>
                            <td>
                                <p>CBCT Scan (Cone Beam Computed Tomography)</p>
                                <p>3D imaging for detailed diagnostics in complex cases.</p>
                            </td>
                            <td>£199</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Teeth Whitening</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>In-Office Whitening</p>
                                <p>Professional teeth whitening performed in the clinic.</p>
                            </td>
                            <td>£549</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Take-Home Whitening Kit</p>
                                <p>Customized kits for at-home teeth whitening.</p>
                            </td>
                            <td>£349</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Whitening Touch-Up</p>
                                <p>Additional sessions for maintaining and enhancing results.</p>
                            </td>
                            <td>£50</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Porcelain Veneers</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Consultation and Assessment</p>
                                <p>Comprehensive evaluation and discussion of veneer options.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Porcelain Veneer Installation</p>
                                <p>Custom-crafted veneers applied to enhance the appearance of teeth.</p>
                            </td>
                            <td>£695 / tooth</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Veneer Replacement</p>
                                <p>Removal and replacement of existing veneers.</p>
                            </td>
                            <td>£695 / tooth</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Smile Makeovers</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Comprehensive Smile Assessment</p>
                                <p>Thorough evaluation to determine the best makeover approach.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Composite Bonding</p>
                                <p>Reshaping and enhancing teeth using composite material.</p>
                            </td>
                            <td>£299 / tooth</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Orthodontic Consultation</p>
                                <p>Evaluation and discussion of orthodontic treatment options.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Aesthetic Treatments</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Comprehensive Smile Assessment</p>
                                <p>Thorough evaluation to determine the best makeover approach.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Composite Bonding</p>
                                <p>Reshaping and enhancing teeth using composite material.</p>
                            </td>
                            <td>£299 / tooth</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Orthodontic Consultation</p>
                                <p>Evaluation and discussion of orthodontic treatment options.</p>
                            </td>
                            <td>Free</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Anti-Wrinkle Injections</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Botox and Dermal Fillers</p>
                                <p>Safe, non-surgical treatments to reduce wrinkles and enhance facial features using premium injectable products.</p>
                            </td>
                            <td>From £199</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Forehead/Crows Feet/Frown Lines</p>
                                <p>Smooth horizontal forehead lines, crow's feet around the eyes, and vertical frown lines between the brows. One area: £200, two areas: £250, three areas: £300</p>
                            </td>
                            <td>£200</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Brow Lift</p>
                                <p>Lift and open the eye area by relaxing muscles that pull the brows downward.</p>
                            </td>
                            <td>£200</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Bunny Lines</p>
                                <p>Eliminate fine lines that appear on the sides of the nose when smiling or scrunching.</p>
                            </td>
                            <td>£180</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Dimpled Chin</p>
                                <p>Eliminate fine lines that appear on the sides of the nose when smiling or scrunching.</p>
                            </td>
                            <td>£120</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Peri-oral Lines</p>
                                <p>Reduce fine lines around the mouth and lip area caused by repetitive muscle movement.</p>
                            </td>
                            <td>£200</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Gummy Smile</p>
                                <p>Reduce excessive gum display when smiling by relaxing the upper lip muscles.</p>
                            </td>
                            <td>£280</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Lip Flip</p>
                                <p>Create the appearance of fuller lips by relaxing muscles around the mouth border.</p>
                            </td>
                            <td>£200</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Jaw Slimming</p>
                                <p>Slim the lower face by reducing masseter muscle bulk for a more feminine jawline.</p>
                            </td>
                            <td>£350</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Nefertiti Neck Lift</p>
                                <p>Tighten and lift the neck area by treating platysmal bands for a more defined jawline.</p>
                            </td>
                            <td>£450</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Excessive Sweating</p>
                                <p>Stop excessive sweating in underarms, palms, or feet with targeted injections.</p>
                            </td>
                            <td>£450</td>
                            </tr>
                        </tbody>
                        </table>

                        <h3>Aesthetic & Injectable Treatments</h3>

                        <table>
                        <tbody>
                            <tr>
                            <td>
                                <p>Cheek Enhancement</p>
                                <p>Add volume and definition to cheekbones for a youthful, sculpted appearance.</p>
                            </td>
                            <td>£300</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Chin Enhancement</p>
                                <p>Improve chin projection and facial balance with precise dermal filler placement.</p>
                            </td>
                            <td>£280</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Jaw Contouring</p>
                                <p>Define and strengthen the jawline for a more masculine or sculpted facial profile.</p>
                            </td>
                            <td>£400</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Lip Enhancement</p>
                                <p>Create fuller, more defined lips with natural-looking volume and shape enhancement.</p>
                            </td>
                            <td>£300</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Non-surgical Rhinoplasty</p>
                                <p>Reshape and refine the nose without surgery using carefully placed dermal fillers.</p>
                            </td>
                            <td>£380</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Temple Rejuvenation</p>
                                <p>Restore youthful volume to hollow temples for improved facial harmony.</p>
                            </td>
                            <td>£400</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Tear Trough Filler</p>
                                <p>Reduce under-eye bags and dark circles by smoothing the tear trough area.</p>
                            </td>
                            <td>£430</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Deep Lines Rejuvenation</p>
                                <p>Target deep facial lines and wrinkles for smoother, younger-looking skin.</p>
                            </td>
                            <td>£250</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Peri-oral Lines</p>
                                <p>Smooth lines around the mouth area, including smoker's lines and lip lines.</p>
                            </td>
                            <td>£250</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Nasolabial Lines</p>
                                <p>Soften smile lines that run from the nose to the corners of the mouth.</p>
                            </td>
                            <td>£300</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Marionette Lines</p>
                                <p>Reduce downward lines from the corners of the mouth to the chin.</p>
                            </td>
                            <td>£300</td>
                            </tr>
                            <tr>
                            <td>
                                <p>Filler Dissolving</p>
                                <p>Safe removal of unwanted or poorly placed dermal filler using hyaluronidase enzyme.</p>
                            </td>
                            <td>£280</td>
                            </tr>
                        </tbody>
                        </table>
                        
                    </div>
                </section>
                
                
                
                
                {open_times}
 
            </main>
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
        open_times = &site.sections["open_times"],
        
    );
    
    css(site);
    
    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("pricing", r##"
    
    {}
        
        main.pricing {
            
            
            
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
            
            
            section.pricing-tables {
            
                .inner {
                
                    margin: 0 auto;
                    
                    padding: 0 var(--site-padding-x);
                    
                    
                    max-width: 1200px;
                
                    h2 {
                        font-size: 28px;
                        color: var(--turquoise-15);
                        margin: 120px 0 48px;
                        
                        @media (max-width: 768px) {
                            font-size: 24px;
                        }
                    }
                        
                    h3 {
                        font-size: 18px;
                        color: var(--turquoise-15);
                        margin-bottom: 24px;
                    }
                    
                    
                    table {
                    
                        width: 100%;
                        
                        letter-spacing: -0.05em;
                        padding: 0 0 48px 48px;
                        
                        box-sizing: border-box;
                        
                        @media (max-width: 768px) {
                        
                            padding: 0 0 48px 0;
                        
                        }
                        
                        
                        
                        
                        
                    
                        tr {
                        
                            display: flex;
                            justify-content: space-between;
                            
                            border-bottom: 1px solid rgb(250, 250, 250);
                            
                            &:last-child {
                                border: none;
                            }
                        
                            p {
                            
                                margin-bottom: 8px;
                                
                                &:last-child {
                                
                                    font-size: 14px;
                                
                                    margin: 0;
                                    
                                    color: #808080;
                                
                                }
                            
                            }
                            
                            td {
                            
                                padding: 11px 0;
                                
                                &:last-child {
                                    text-align: right;
                                    min-width: 120px;
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