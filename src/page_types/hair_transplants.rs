use crate::prelude::*;

pub fn construct_hair_transplants(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    add_default_og_image(page);
    let head = site.construct_head(page);

    let auto_slider_script = auto_slider_script();

    let html = format!(
        r##"
        <!DOCTYPE html>
        <html lang="en-GB">
        {head}
        <body>
            {header}
            <main class="hair-transplants">

                <section class="hero transparent-header">
                    <div class="content">
                        <div class="text">
                            <h1>Expert FUE Hair Transplant</h1>
                            <p>Natural-looking results from experienced specialists in Manchester and Leeds</p>
                            <div class="full-price">£6,000</div>
                            <div class="price">£2,500</div>
                            <p class="price-detail">Includes PRP treatment | 3,500-4,000 grafts</p>
                            <a href="#consultation" class="cta-button">Book a Free Video Consultation</a>
                        </div>
                        <div class="auto-slider">
                            <img class="before" src="/images/hair-transplants/1b.webp" alt="Before">
                            <img class="after" src="/images/hair-transplants/1a.webp" alt="After">
                            <!--<div class="labels">
                                <span>BEFORE</span>
                                <span>AFTER</span>
                            </div>-->
                        </div>
                    </div>
                </section>

                <section class="about">
                    <h2>Expert Hair Restoration in Manchester & Leeds</h2>
                    <p>We provide effective hair restoration treatments helping men and women across the North of England to say goodbye to hair loss.</p>
                    
                    <div class="features">
                        <ul>
                            <li><strong>Flexible Payment Plans</strong> available</li>
                            <li><strong>Free Consultation</strong> with guided explanation of procedure</li>
                        </ul>
                        <ul>
                            <li>Supportive <strong>After-Care Programme</strong> with all products provided</li>
                            <li>Full range of <strong>guaranteed</strong> hair loss treatments</li>
                        </ul>
                    </div>
                </section>

                <section class="guarantee">
                    <div class="container">
                        <h2>Guaranteed Results with Advanced FUE Technique</h2>
                        
                        <div class="guarantee-content">
                            <div class="guarantee-text">
                                <h3>Our Satisfaction Guarantee</h3>
                                <p>We don't just promise results - we guarantee them. Every hair transplant procedure at UrgentCare Dental comes with our satisfaction guarantee. Our surgeons have successfully completed thousands of procedures, achieving natural-looking results that restore both hair and confidence. We stand behind every transplant we perform.</p>

                                <h3>The Gold Standard: FUE Hair Transplantation</h3>
                                <p>Our Follicular Unit Extraction (FUE) method is the gold standard in hair restoration. Unlike older strip methods, FUE leaves no linear scarring and offers faster recovery times. We extract individual hair follicles from donor areas and transplant them with precision to create a natural hairline and optimal density.</p>

                                <p>Each graft is carefully handled to ensure maximum survival rate - our advanced techniques achieve graft survival rates of 95-98%, meaning almost every transplanted follicle successfully takes root and grows. With 3,500-4,000 grafts per procedure, we provide significantly more coverage than many clinics offer at this price point.</p>
                            </div>

                            <div class="guarantee-stats">
                                <div class="stat-card">
                                    <div class="stat-number">3,500-4,000</div>
                                    <div class="stat-label">Grafts Per Procedure</div>
                                </div>
                                <div class="stat-card">
                                    <div class="stat-number">95-98%</div>
                                    <div class="stat-label">Graft Survival Rate</div>
                                </div>
                                <div class="stat-card">
                                    <div class="stat-number">6-8 Hours</div>
                                    <div class="stat-label">Procedure Time</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="track-record">
                    <div class="container">
                        <div class="track-content">
                            <div class="track-image">
                                <img src="/images/hair-transplants/hair-8457142_1280.jpg" alt="Expert Care">
                            </div>
                            <div class="track-text">
                                <h2>A Track Record You Can Trust</h2>
                                <p>UrgentCare Dental has been serving the community for years, building a reputation for clinical excellence and patient-centred care. Our expansion into hair restoration brings the same rigorous standards to this new service. Every member of our hair transplant team is fully qualified, extensively trained, and committed to achieving outstanding results.</p>

                                <h3>Comprehensive Aftercare Programme</h3>
                                <p>Your success doesn't end when you leave our clinic. We provide a complete aftercare kit including specialised shampoo, healing solutions, and detailed recovery instructions. Our team remains available throughout your recovery period to answer questions and provide guidance.</p>

                                <p>We include PRP therapy with every procedure to enhance healing and accelerate growth - a treatment that typically costs £500-800 as a standalone service. We schedule check-ins at key milestones and offer a complimentary 12-month review to document your transformation and ensure you're delighted with your results.</p>

                                <div class="track-features">
                                    <div class="feature">
                                        <strong>✓ Complete Aftercare Kit</strong>
                                        <span>Specialised products included</span>
                                    </div>
                                    <div class="feature">
                                        <strong>✓ PRP Therapy Included</strong>
                                        <span>Worth £500-800 standalone</span>
                                    </div>
                                    <div class="feature">
                                        <strong>✓ 12-Month Follow-Up</strong>
                                        <span>Complimentary review appointment</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="locations-cta">
                    <div class="container">
                        <div class="locations-content">
                            <div class="locations-text">
                                <h2>Convenient Locations, Exceptional Service</h2>
                                
                                <h3>Manchester & Leeds Clinics</h3>
                                <p>With clinics in both Manchester and Leeds, we're easily accessible for patients across Yorkshire, Lancashire, and the wider North of England. Our modern facilities are equipped with the latest technology, and our skin aestheticians are on-site to support both your procedure and recovery.</p>

                                <h3>Free, No-Obligation Consultation</h3>
                                <p>Your journey begins with a thorough consultation where we assess your hair loss pattern, discuss your goals, and create a personalised treatment plan. Available every Monday evening from 6pm-10pm via video call, our 30-minute consultations are completely free with no pressure or obligation.</p>

                                <p>We understand that choosing hair restoration is a significant decision. Our consultations are designed to answer all your questions, explain the procedure in detail, and help you understand what results you can realistically expect. We'll discuss your medical history, examine your hair loss pattern, and determine if you're a suitable candidate for FUE hair transplant.</p>

                                <h3>Transparent, All-Inclusive Pricing</h3>
                                <p>At £2,500, our FUE hair transplant package includes everything: consultation, 3,500-4,000 grafts, PRP treatment, complete aftercare products, and 12-month follow-up. No hidden fees, no surprises. We believe in transparent pricing so you can make an informed decision without worrying about unexpected costs.</p>
                            </div>

                            <div class="locations-cta-box">
                                <h3>Book Your Free Consultation</h3>
                                <div class="cta-details">
                                    <p><strong>Available:</strong> Every Monday</p>
                                    <p><strong>Time:</strong> 6pm - 10pm</p>
                                    <p><strong>Duration:</strong> 30 minutes</p>
                                    <p><strong>Format:</strong> Video call</p>
                                    <p><strong>Cost:</strong> Completely free</p>
                                </div>
                                
                                <a href="{BOOKING_LINK}" class="cta-button">Book Online</a>
                                <p class="or">or</p>
                                <a href="{PHONE_NUMBER_LINK}" class="cta-button">Call 0113 868 3185</a>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="results">
                    <h2>Real Patient Results</h2>
                    <p class="subtitle">Drag the slider to compare before and after</p>
                    <div class="grid">
                        <div class="manual-slider">
                            <img class="before" src="/images/hair-transplants/2b.webp" alt="Before">
                            <img class="after" src="/images/hair-transplants/2a.webp" alt="After">
                            <div class="handle"></div>
                        </div>

                        <div class="manual-slider">
                            <img class="before" src="/images/hair-transplants/3b.webp" alt="Before">
                            <img class="after" src="/images/hair-transplants/3a.webp" alt="After">
                            <div class="handle"></div>
                        </div>

                        <div class="manual-slider">
                            <img class="before" src="/images/hair-transplants/4b.webp" alt="Before">
                            <img class="after" src="/images/hair-transplants/4a.webp" alt="After">
                            <div class="handle"></div>
                        </div>

                        <div class="manual-slider">
                            <img class="before" src="/images/hair-transplants/5b.webp" alt="Before">
                            <img class="after" src="/images/hair-transplants/5a.webp" alt="After">
                            <div class="handle"></div>
                        </div>

                        <div class="manual-slider">
                            <img class="before" src="/images/hair-transplants/6b.webp" alt="Before">
                            <img class="after" src="/images/hair-transplants/6a.webp" alt="After">
                            <div class="handle"></div>
                        </div>

                        <div class="manual-slider">
                            <img class="before" src="/images/hair-transplants/7b.webp" alt="Before">
                            <img class="after" src="/images/hair-transplants/7a.webp" alt="After">
                            <div class="handle"></div>
                        </div>
                    </div>
                </section>

                <section class="process">
                    <div class="container">
                        <h2>The Hair Transplant Process</h2>
                        <div class="steps">
                            <div class="step">
                                <div class="number">1</div>
                                <h3>Free Consultation</h3>
                                <p>We'll conduct a complimentary consultation to assess your hair loss and discuss your restoration goals.</p>
                            </div>
                            <div class="step">
                                <div class="number">2</div>
                                <h3>Personalised Plan</h3>
                                <p>Our specialists create a customised treatment plan tailored to your specific needs and desired results.</p>
                            </div>
                            <div class="step">
                                <div class="number">3</div>
                                <h3>FUE Procedure</h3>
                                <p>The procedure takes 6-8 hours as our team carefully extracts and transplants 3,500-4,000 healthy hair grafts.</p>
                            </div>
                            <div class="step">
                                <div class="number">4</div>
                                <h3>PRP Treatment</h3>
                                <p>Included PRP therapy enhances graft survival and promotes faster, healthier hair growth.</p>
                            </div>
                            <div class="step">
                                <div class="number">5</div>
                                <h3>Aftercare Support</h3>
                                <p>Full aftercare programme with all products provided and ongoing support throughout your recovery.</p>
                            </div>
                            <div class="step">
                                <div class="number">6</div>
                                <h3>12 Month Review</h3>
                                <p>Complimentary follow-up appointment to review your results and ensure your satisfaction.</p>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="benefits">
                    <h2>Why Choose Our FUE Hair Transplant?</h2>
                    <div class="grid">
                        <div class="card">
                            <div class="icon">✓</div>
                            <h3>Advanced FUE Technique</h3>
                            <p>Minimally invasive procedure with natural-looking results and faster recovery</p>
                        </div>
                        <div class="card">
                            <div class="icon">👨‍⚕️</div>
                            <h3>Expert Specialists</h3>
                            <p>Experienced surgeons with proven track record in hair restoration</p>
                        </div>
                        <div class="card">
                            <div class="icon">💉</div>
                            <h3>PRP Included</h3>
                            <p>Platelet-Rich Plasma treatment included to enhance graft survival and growth</p>
                        </div>
                        <div class="card">
                            <div class="icon">📍</div>
                            <h3>Convenient Locations</h3>
                            <p>Clinics in Manchester and Leeds with skin aestheticians on-site</p>
                        </div>
                        <div class="card">
                            <div class="icon">💰</div>
                            <h3>Transparent Pricing</h3>
                            <p>£2,500 all-inclusive for 3,500-4,000 grafts with PRP treatment</p>
                        </div>
                        <div class="card">
                            <div class="icon">⏰</div>
                            <h3>Free Consultation</h3>
                            <p>30-minute e-consultation to assess your needs and answer questions</p>
                        </div>
                    </div>
                </section>

                <section class="consultation">
                    <span id="consultation"></span>
                    <div class="container">
                        <h2>Book Your Free E-Consultation</h2>
                        <div class="details">
                            <p><strong>Available:</strong> Every Monday, 6pm - 10pm</p>
                            <p><strong>Duration:</strong> 30 minutes</p>
                            <p><strong>Format:</strong> Video consultation</p>
                            <p style="margin-top: 2rem;">Discuss your hair restoration goals, get expert advice, and learn if you're a suitable candidate for FUE hair transplant.</p>
                        </div>
                        <a href="{BOOKING_LINK}" class="cta-button">Book Online</a>
                        <p style="margin: 1rem 0;">or</p>
                        <a href="{PHONE_NUMBER_LINK}" class="cta-button">Call 0113 868 3185</a>
                    </div>
                </section>
            </main>
            {auto_slider_script}
            {footer}
        </body>
        </html>
    "##,
        header = construct_header(site, &page.foundation),
        footer = construct_footer(site),
    );

    css(site);

    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("hair_transplants", r##"
    
    {}
        
        main.hair-transplants {
        
            line-height: 1.6;

            .hero {
                background: linear-gradient(135deg, #0a5959 0%, #0d7070 100%);
                color: white;
                padding: 100px 5% 4rem;
                
                @media (max-width: 768px) {
                    padding: 140px 5% 4rem;
                }

                .content {
                    max-width: 1200px;
                    margin: 0 auto;
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 3rem;
                    align-items: center;
                }

                .text {
                    h1 {
                        font-size: 2.5rem;
                        margin-bottom: 1rem;
                        line-height: 1.2;
                        color: white;
                        font-weight: 700;
                    }

                    p {
                        font-size: 1.1rem;
                        margin-bottom: 1.5rem;
                        opacity: 0.95;
                    }
                    
                    .full-price {
                        font-size: 1.8rem;
                        font-weight: 700;
                        margin: 0 0 0;
                        
                        position: relative;
                        display: inline-block;
                        color: var(--turquoise-30);
                        
                        &::after {
                            content: '';
                            position: absolute;
                            left: 0;
                            top: 50%;
                            width: 100%;
                            height: 3px;
                            background: currentColor;
                            transform: translateY(-50%) rotate(-10deg);
                        }
                    }


                    .price {
                        font-size: 2.5rem;
                        font-weight: 700;
                        /*margin: 1.5rem 0 0.5rem;*/
                        margin: 0 0 1.5rem 0;
                    }

                    .price-detail {
                        font-size: 1rem;
                        opacity: 0.9;
                        margin-bottom: 2rem;
                    }
                    
                    @media (max-width: 768px) {
                    
                        h1 {
                            font-size: 1.8rem;
                        }
                        
                        .full-price {
                            font-size: 1.44rem;
                        }

                        .price {
                            font-size: 2rem;
                        }
                    
                    }

                    .cta-button {
                        background: #00bfb3;
                        color: white;
                        padding: 1rem 2rem;
                        border: none;
                        border-radius: 4px;
                        font-size: 1.1rem;
                        font-weight: 600;
                        cursor: pointer;
                        text-decoration: none;
                        display: inline-block;
                        transition: background 0.3s;

                        &:hover {
                            background: #00a89d;
                        }
                    }
                }

                .auto-slider {
                    position: relative;
                    width: 100%;
                    aspect-ratio: 1;
                    border-radius: 12px;
                    overflow: hidden;
                    box-shadow: 0 8px 24px rgba(0,0,0,0.3);

                    img {
                        position: absolute;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        object-fit: cover;
                        user-select: none;
                        -webkit-user-drag: none;
                        pointer-events: none;
                    }

                    .after {
                        clip-path: inset(0 0 0 0%);
                        transition: clip-path 1s ease-in-out;
                    }

                    .labels {
                        position: absolute;
                        bottom: 0;
                        left: 0;
                        right: 0;
                        display: flex;
                        justify-content: space-between;
                        padding: 1rem;
                        background: linear-gradient(transparent, rgba(0,0,0,0.5));
                        font-weight: 600;
                        font-size: 0.9rem;
                        pointer-events: none;
                    }
                }
            }

            .about {
                padding: 4rem 5%;
                max-width: 1200px;
                margin: 0 auto;
                text-align: center;

                h2 {
                    color: #0a5959;
                    font-size: 2rem;
                    margin-bottom: 2rem;
                }

                .features {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 1rem;
                    margin: 2rem 0;
                    text-align: left;

                    ul {
                        list-style: none;
                        padding: 0;

                        li {
                            padding: 0.5rem 0;
                            padding-left: 1.5rem;
                            position: relative;

                            &::before {
                                content: '✓';
                                position: absolute;
                                left: 0;
                                color: #00bfb3;
                                font-weight: bold;
                            }
                        }
                    }
                }
            }

            .guarantee {
                background: white;
                padding: 4rem 5%;

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                h2 {
                    color: #0a5959;
                    font-size: 2rem;
                    margin-bottom: 3rem;
                    text-align: center;
                }

                .guarantee-content {
                    display: grid;
                    grid-template-columns: 2fr 1fr;
                    gap: 3rem;
                }

                .guarantee-text {
                    p {
                        margin-bottom: 1.5rem;
                        line-height: 1.8;
                        color: #555;
                    }

                    h3 {
                        color: #0a5959;
                        font-size: 1.3rem;
                        margin: 2rem 0 1rem;

                        &:first-of-type {
                            margin-top: 0;
                        }
                    }
                }

                .guarantee-stats {
                    display: flex;
                    flex-direction: column;
                    gap: 1.5rem;
                }

                .stat-card {
                    background: #f8f9fa;
                    padding: 2rem;
                    border-radius: 8px;
                    text-align: center;
                    box-shadow: 0 2px 8px rgba(0,0,0,0.1);

                    .stat-number {
                        font-size: 2.5rem;
                        font-weight: 700;
                        color: #00bfb3;
                        margin-bottom: 0.5rem;
                    }

                    .stat-label {
                        color: #0a5959;
                        font-weight: 600;
                    }
                }
            }

            .track-record {
                background: #f8f9fa;
                padding: 4rem 5%;

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                .track-content {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 3rem;
                    align-items: center;
                }

                .track-image {
                    .placeholder-image {
                        width: 100%;
                        aspect-ratio: 1;
                        background: #ddd;
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 1.5rem;
                        color: #999;
                    }
                    img {
                        display: block;
                        width: 100%;
                        aspect-ratio: 1;
                        background: #ddd;
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 1.5rem;
                        color: #999;
                        object-fit: cover;
                    }
                }

                .track-text {
                    h2 {
                        color: #0a5959;
                        font-size: 2rem;
                        margin-bottom: 1.5rem;
                    }

                    h3 {
                        color: #0a5959;
                        font-size: 1.3rem;
                        margin: 2rem 0 1rem;
                    }

                    p {
                        margin-bottom: 1.5rem;
                        line-height: 1.8;
                        color: #555;
                    }
                }

                .track-features {
                    margin-top: 2rem;
                    display: flex;
                    flex-direction: column;
                    gap: 1rem;

                    .feature {
                        background: white;
                        padding: 1rem;
                        border-radius: 8px;
                        display: flex;
                        flex-direction: column;

                        strong {
                            color: #0a5959;
                            margin-bottom: 0.25rem;
                        }

                        span {
                            color: #666;
                            font-size: 0.9rem;
                        }
                    }
                }
            }

            .locations-cta {
                background: white;
                padding: 4rem 5%;

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                .locations-content {
                    display: grid;
                    grid-template-columns: 2fr 1fr;
                    gap: 3rem;
                }

                .locations-text {
                    h2 {
                        color: #0a5959;
                        font-size: 2rem;
                        margin-bottom: 2rem;
                    }

                    h3 {
                        color: #0a5959;
                        font-size: 1.3rem;
                        margin: 2rem 0 1rem;

                        &:first-of-type {
                            margin-top: 0;
                        }
                    }

                    p {
                        margin-bottom: 1.5rem;
                        line-height: 1.8;
                        color: #555;
                    }
                }

                .locations-cta-box {
                    background: #0a5959;
                    color: white;
                    padding: 2rem;
                    border-radius: 12px;
                    height: fit-content;
                    position: sticky;
                    top: 2rem;

                    h3 {
                        color: white;
                        margin-bottom: 1.5rem;
                        text-align: center;
                    }

                    .cta-details {
                        background: rgba(255,255,255,0.1);
                        padding: 1.5rem;
                        border-radius: 8px;
                        margin-bottom: 1.5rem;

                        p {
                            margin: 0.5rem 0;
                            font-size: 0.95rem;
                        }
                    }

                    .cta-button {
                        background: #00bfb3;
                        color: white;
                        padding: 1rem;
                        border: none;
                        border-radius: 4px;
                        font-size: 1rem;
                        font-weight: 600;
                        cursor: pointer;
                        text-decoration: none;
                        display: block;
                        width: 100%;
                        text-align: center;
                        transition: background 0.3s;
                        box-sizing: border-box;

                        &:hover {
                            background: #00a89d;
                        }
                    }

                    .or {
                        text-align: center;
                        margin: 1rem 0;
                        opacity: 0.8;
                    }
                }
            }

            .why-choose {
                background: #f8f9fa;
                padding: 4rem 5%;

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                h2 {
                    color: #0a5959;
                    font-size: 2rem;
                    margin-bottom: 3rem;
                    text-align: center;
                }

                .why-content {
                    display: grid;
                    grid-template-columns: 2fr 1fr;
                    gap: 3rem;
                }

                .why-text {
                    p {
                        margin-bottom: 1.5rem;
                        line-height: 1.8;
                        color: #555;
                    }

                    h3 {
                        color: #0a5959;
                        font-size: 1.3rem;
                        margin: 2rem 0 1rem;

                        &:first-of-type {
                            margin-top: 0;
                        }
                    }
                }

                .why-stats {
                    display: flex;
                    flex-direction: column;
                    gap: 1.5rem;
                }

                .stat-card {
                    background: white;
                    padding: 2rem;
                    border-radius: 8px;
                    text-align: center;
                    box-shadow: 0 2px 8px rgba(0,0,0,0.1);

                    .stat-number {
                        font-size: 2.5rem;
                        font-weight: 700;
                        color: #00bfb3;
                        margin-bottom: 0.5rem;
                    }

                    .stat-label {
                        color: #0a5959;
                        font-weight: 600;
                    }
                }
            }

            .why-choose {
                background: #f8f9fa;
                padding: 4rem 5%;

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                h2 {
                    color: #0a5959;
                    font-size: 2rem;
                    margin-bottom: 3rem;
                    text-align: center;
                }

                .why-content {
                    display: grid;
                    grid-template-columns: 2fr 1fr;
                    gap: 3rem;
                }

                .why-text {
                    p {
                        margin-bottom: 1.5rem;
                        line-height: 1.8;
                        color: #555;
                    }

                    h3 {
                        color: #0a5959;
                        font-size: 1.3rem;
                        margin: 2rem 0 1rem;

                        &:first-of-type {
                            margin-top: 0;
                        }
                    }
                }

                .why-stats {
                    display: flex;
                    flex-direction: column;
                    gap: 1.5rem;
                }

                .stat-card {
                    background: white;
                    padding: 2rem;
                    border-radius: 8px;
                    text-align: center;
                    box-shadow: 0 2px 8px rgba(0,0,0,0.1);

                    .stat-number {
                        font-size: 2.5rem;
                        font-weight: 700;
                        color: #00bfb3;
                        margin-bottom: 0.5rem;
                    }

                    .stat-label {
                        color: #0a5959;
                        font-weight: 600;
                    }
                }
            }

            .results {
                padding: 4rem 5%;
                max-width: 1200px;
                margin: 0 auto;

                h2 {
                    color: #0a5959;
                    font-size: 2rem;
                    margin-bottom: 1rem;
                    text-align: center;
                }

                .subtitle {
                    text-align: center;
                    color: #666;
                    margin-bottom: 3rem;
                }

                .grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 2rem;
                }

                .manual-slider {
                    position: relative;
                    width: 100%;
                    aspect-ratio: 1;
                    overflow: hidden;
                    border-radius: 8px;
                    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                    cursor: ew-resize;
                    user-select: none;
                    touch-action: none;

                    img {
                        position: absolute;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        object-fit: cover;
                        user-select: none;
                        -webkit-user-drag: none;
                        pointer-events: none;
                    }

                    .after {
                        clip-path: inset(0 0 0 50%);
                    }

                    .handle {
                        position: absolute;
                        top: 0;
                        left: 50%;
                        width: 3px;
                        height: 100%;
                        background: white;
                        transform: translateX(-50%);
                        pointer-events: none;
                        box-shadow: 0 0 8px rgba(0,0,0,0.3);

                        &::before {
                            content: '';
                            position: absolute;
                            top: 50%;
                            left: 50%;
                            transform: translate(-50%, -50%);
                            width: 40px;
                            height: 40px;
                            background: white;
                            border-radius: 50%;
                            box-shadow: 0 2px 8px rgba(0,0,0,0.2);
                            /*background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="%230a5959"><path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/><path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/></svg>');
                            background-size: 60%;
                            background-position: center;
                            background-repeat: no-repeat;*/
                        }
                        
                        &::after {
                            content: '◀ ▶';
                            position: absolute;
                            top: 50%;
                            left: 50%;
                            transform: translate(-50%, -50%);
                            font-size: 12px;
                            color: #0a5959;  // or your teal color
                        }
                    }
                }
            }

            .process {
                background: #f8f9fa;
                padding: 4rem 5%;

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                h2 {
                    color: #0a5959;
                    font-size: 2rem;
                    margin-bottom: 3rem;
                    text-align: center;
                }

                .steps {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 2rem;
                }

                .step {
                    background: white;
                    padding: 2rem;
                    border-radius: 8px;
                    box-shadow: 0 2px 8px rgba(0,0,0,0.1);

                    .number {
                        display: inline-block;
                        width: 40px;
                        height: 40px;
                        background: #00bfb3;
                        color: white;
                        border-radius: 50%;
                        text-align: center;
                        line-height: 40px;
                        font-weight: bold;
                        margin-bottom: 1rem;
                    }

                    h3 {
                        color: #0a5959;
                        margin-bottom: 0.5rem;
                    }
                }
            }

            .benefits {
                padding: 4rem 5%;
                max-width: 1200px;
                margin: 0 auto;

                h2 {
                    color: #0a5959;
                    font-size: 2rem;
                    margin-bottom: 2rem;
                    text-align: center;
                }

                .grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 2rem;
                }

                .card {
                    background: #f8f9fa;
                    padding: 2rem;
                    border-radius: 8px;
                    text-align: center;

                    .icon {
                        width: 80px;
                        height: 80px;
                        margin: 0 auto 1.5rem;
                        background: linear-gradient(135deg, #00bfb3 0%, #0a5959 100%);
                        border-radius: 12px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 2rem;
                        color: white;
                        box-shadow: 0 4px 12px rgba(0,191,179,0.3);
                    }

                    h3 {
                        color: #0a5959;
                        margin-bottom: 0.5rem;
                    }
                }
            }

            .consultation {
                background: #0a5959;
                color: white;
                text-align: center;
                padding: 4rem 5%;
                position: relative;
                
                #consultation {
                    position: absolute;
                    top: -100px;
                }

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                h2 {
                    color: white;
                    font-size: 2rem;
                    margin-bottom: 2rem;
                }

                .details {
                    background: rgba(255,255,255,0.1);
                    padding: 2rem;
                    border-radius: 8px;
                    margin: 2rem auto;
                    max-width: 600px;

                    p {
                        font-size: 1.1rem;
                        margin: 0.5rem 0;
                    }
                }

                .cta-button {
                    background: #00bfb3;
                    color: white;
                    padding: 1rem 2rem;
                    border: none;
                    border-radius: 4px;
                    font-size: 1.1rem;
                    font-weight: 600;
                    cursor: pointer;
                    text-decoration: none;
                    display: inline-block;
                    margin: 0.5rem;
                    transition: background 0.3s;

                    &:hover {
                        background: #00a89d;
                    }
                }
            }

            .footer {
                background: #0a5959;
                color: white;
                padding: 3rem 5% 2rem;

                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                .footer-content {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                    gap: 2rem;
                    margin-bottom: 2rem;
                }

                h4 {
                    color: #00bfb3;
                    margin-bottom: 1rem;
                }

                ul {
                    list-style: none;
                    padding: 0;

                    li {
                        margin: 0.5rem 0;

                        a {
                            color: white;
                            text-decoration: none;
                            opacity: 0.9;
                            transition: opacity 0.3s;

                            &:hover {
                                opacity: 1;
                            }
                        }
                    }
                }

                .copyright {
                    text-align: center;
                    padding-top: 2rem;
                    border-top: 1px solid rgba(255,255,255,0.1);
                    opacity: 0.8;
                }
            }

            @media (max-width: 768px) {
                .hero .content {
                    grid-template-columns: 1fr;
                    gap: 2rem;
                
                }

                .about .features {
                    grid-template-columns: 1fr;
                }

                .why-choose .why-content {
                    grid-template-columns: 1fr;
                }

                .guarantee {
                
                    .guarantee-content {
                        grid-template-columns: 1fr;
                    }

                    .guarantee-stats {
                        grid-template-columns: repeat(3, 1fr);
                        flex-direction: row;
                        
                    }

                    .stat-card {
                        padding: 1.5rem 1rem;

                        .stat-number {
                            font-size: 1.8rem;
                        }

                        .stat-label {
                            font-size: 0.85rem;
                        }
                    }
                }


                .track-record {
                
                
                    .track-content {
                        grid-template-columns: 1fr;
                    }

                    .track-image {
                        order: -1;
                    }
                }

                .locations-cta  {
                
                    .locations-content {
                        grid-template-columns: 1fr;
                    }
                    .locations-cta-box {
                        position: static;
                    }
                }

                .results h2,
                .process h2,
                .benefits h2,
                .consultation h2 {
                    font-size: 1.5rem;
                }

                .results .grid {
                    grid-template-columns: 1fr;
                }

                .process .steps {
                    grid-template-columns: 1fr;
                }
            }
        }
        
    "##);
}

fn auto_slider_script() -> String {
    r##"
        <script>
            // Auto-slider with pause (1s slide, 1s pause)
            const autoSlider = document.querySelector('.auto-slider');
            const autoAfter = autoSlider.querySelector('.after');
            
            let isForward = true;
            
            function animateAutoSlider() {
                if (isForward) {
                    autoAfter.style.clipPath = 'inset(0 0 0 100%)';
                } else {
                    autoAfter.style.clipPath = 'inset(0 0 0 0%)';
                }
                
                isForward = !isForward;
            }
            
            // Start after initial pause
            setTimeout(() => {
                animateAutoSlider();
                setInterval(animateAutoSlider, 2000); // 1s animation + 1s pause
            }, 1000);

            // Manual sliders
            document.querySelectorAll('.manual-slider').forEach(slider => {
                const after = slider.querySelector('.after');
                const handle = slider.querySelector('.handle');
                let isDragging = false;

                function updatePosition(x) {
                    const rect = slider.getBoundingClientRect();
                    const offsetX = x - rect.left;
                    const percentage = Math.max(0, Math.min(100, (offsetX / rect.width) * 100));
                    
                    handle.style.left = percentage + '%';
                    after.style.clipPath = `inset(0 0 0 ${percentage}%)`;
                }

                slider.addEventListener('mousedown', (e) => {
                    isDragging = true;
                    updatePosition(e.clientX);
                });

                document.addEventListener('mousemove', (e) => {
                    if (isDragging) {
                        e.preventDefault();
                        updatePosition(e.clientX);
                    }
                });

                document.addEventListener('mouseup', () => {
                    isDragging = false;
                });

                slider.addEventListener('touchstart', (e) => {
                    isDragging = true;
                    updatePosition(e.touches[0].clientX);
                });

                slider.addEventListener('touchmove', (e) => {
                    if (isDragging) {
                        e.preventDefault();
                        updatePosition(e.touches[0].clientX);
                    }
                });

                slider.addEventListener('touchend', () => {
                    isDragging = false;
                });
            });
        </script>
    "##
    .to_owned()
}
