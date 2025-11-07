use crate::prelude::*;

pub fn construct_implants(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    let head = site.construct_head(page);
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-US">
        {head}
        <body>
            {header}
            <main class="implants">
            
            
                <section class="dentist-splash-hero">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="text-area">
                        <h1>Dental Implants<br>That Last a Lifetime</h1>
                        <p class="price">From £1,999 per implant</p>
                        <p>Replace missing teeth with the most advanced implant technology. Natural-looking, long term results with transparent pricing and 0% finance available.</p>
                        <div class="buttons">
                            <a href="{BOOKING_LINK}" class="primary">Book a Free Consultation</a>
                            <a href="{PHONE_NUMBER_LINK}" class="secondary">Call {PHONE_NUMBER}</a>
                            <a href="https://wa.me/447383502032" class="secondary">WhatsApp Us</a>
                        </div>
                    </div>
                    <div class="person-area">
                        <div class="shadow"></div>
                        <img src="/images/dentists/Dentist.png" alt="Dentist at UrgentCare Dental" />
                    </div>
                </section>
                
                <section class="benefits">
                    <div class="container">
                        <h2>Why Choose Dental Implants?</h2>
                        <div class="benefits-grid">
                            <div class="benefit-card">
                                <div class="icon">🦷</div>
                                <h3>Permanent Solution</h3>
                                <p>Unlike dentures or bridges, implants are a long term replacement that can last 25+ years with proper care.</p>
                            </div>
                            <div class="benefit-card">
                                <div class="icon">✨</div>
                                <h3>Flawless Aesthetics</h3>
                                <p>Implants look, feel, and function exactly like your natural teeth. No one will know the difference.</p>
                            </div>
                            <div class="benefit-card">
                                <div class="icon">💪</div>
                                <h3>Full Chewing Power</h3>
                                <p>Restore 100% of your bite force. Eat steaks and crusty bread - implants are as strong as natural teeth.</p>
                            </div>
                            <div class="benefit-card">
                                <div class="icon">😊</div>
                                <h3>Maintains Facial Structure</h3>
                                <p>Prevents the sunken face appearance that happens with missing teeth. Implants preserve your jawbone and facial proportions.</p>
                            </div>
                            <div class="benefit-card">
                                <div class="icon">🎯</div>
                                <h3>No Damage to Other Teeth</h3>
                                <p>Unlike bridges that grind down healthy teeth, implants stand alone without affecting your remaining teeth.</p>
                            </div>
                            <div class="benefit-card">
                                <div class="icon">⏰</div>
                                <h3>Quick Recovery</h3>
                                <p>Most patients return to work in 2-3 days. Modern techniques mean less discomfort than a typical extraction, with minimal swelling or bruising.</p>
                            </div>
                        </div>
                    </div>
                </section>
                
                <section class="process">
                    <div class="container">
                        <h2>Your Journey to a New Smile</h2>
                        <p class="process-intro">Our streamlined implant process ensures minimal disruption to your life with maximum results</p>
                        <div class="timeline">
                            <div class="timeline-item">
                                <div class="timeline-dot"></div>
                                <h3>Initial Consultation</h3>
                                <p class="timeline-duration">Same week booking</p>
                                <p>Comprehensive examination, 3D imaging, and personalized treatment plan with transparent pricing.</p>
                            </div>
                            <div class="timeline-item">
                                <div class="timeline-dot"></div>
                                <h3>Implant Placement</h3>
                                <p class="timeline-duration">90-minute procedure</p>
                                <p>Precision placement of the titanium implant using advanced guided surgery techniques.</p>
                            </div>
                            <div class="timeline-item">
                                <div class="timeline-dot"></div>
                                <h3>Healing Period</h3>
                                <p class="timeline-duration">3-4 months</p>
                                <p>The implant fuses with your jawbone while you wear a temporary tooth if needed.</p>
                            </div>
                            <div class="timeline-item">
                                <div class="timeline-dot"></div>
                                <h3>Final Crown</h3>
                                <p class="timeline-duration">2 appointments</p>
                                <p>Custom-made porcelain crown matched perfectly to your natural teeth for a seamless smile.</p>
                            </div>
                        </div>
                    </div>
                </section>
                
                <section class="pricing" id="pricing">
                    <div class="container">
                        <h2>Transparent Implant Pricing</h2>
                        <div class="pricing-cards">
                            <div class="pricing-card">
                                <h3>Single Implant</h3>
                                <div class="price">£1,999</div>
                                <p class="price-note">Complete package price</p>
                                <ul>
                                    <li>Titanium implant</li>
                                    <li>Abutment included</li>
                                    <li>Porcelain crown</li>
                                    <li>All appointments</li>
                                    <li>Aftercare included</li>
                                </ul>
                                <a href="{BOOKING_LINK}" class="book-btn">Book Now</a>
                            </div>
                            <div class="pricing-card featured">
                                <span class="badge">Most Popular</span>
                                <h3>Multiple Implants</h3>
                                <div class="price">£1,799</div>
                                <p class="price-note">Per implant (3+ teeth)</p>
                                <ul>
                                    <li>Everything included</li>
                                    <li>10% multi-tooth discount</li>
                                    <li>Single surgery session</li>
                                    <li>Priority scheduling</li>
                                    <li>Extended warranty</li>
                                </ul>
                                <a href="{BOOKING_LINK}" class="book-btn">Book Now</a>
                            </div>
                            <div class="pricing-card">
                                <h3>All-on-4</h3>
                                <div class="price">£12,999</div>
                                <p class="price-note">Full arch restoration</p>
                                <ul>
                                    <li>4-6 implants per arch</li>
                                    <li>Fixed full bridge</li>
                                    <li>Same-day teeth option</li>
                                    <li>Bone grafting if needed</li>
                                    <li>Lifetime support</li>
                                </ul>
                                <a href="{BOOKING_LINK}" class="book-btn">Book Now</a>
                            </div>
                        </div>
                    </div>
                </section>
                
                <section class="faq">
                    <div class="container">
                        <h2>Common Questions About Implants</h2>
                        <div class="faq-container">
                            <div class="faq-item">
                                <div class="faq-question">How long do dental implants last?</div>
                                <div class="faq-answer">With proper care, dental implants can last 25+ years or even a lifetime. The crown may need replacement after 10-15 years due to normal wear, but the implant itself is permanent. Studies show a 95% success rate after 10 years.</div>
                            </div>
                            <div class="faq-item">
                                <div class="faq-question">Is the implant procedure painful?</div>
                                <div class="faq-answer">Most patients report less discomfort than expected. The procedure is done under local anesthesia, and many say it's more comfortable than a tooth extraction. We provide comprehensive pain management and most patients return to work within 2-3 days.</div>
                            </div>
                            <div class="faq-item">
                                <div class="faq-question">Am I suitable for implants?</div>
                                <div class="faq-answer">Most adults with good general health are suitable candidates. We'll assess your bone density with 3D imaging during consultation. Even if you've been told you lack bone, modern grafting techniques can often make implants possible.</div>
                            </div>
                            <div class="faq-item">
                                <div class="faq-question">What financing options are available?</div>
                                <div class="faq-answer">We offer 0% APR financing for 12-24 months on treatments over £500. Monthly payments start from £85 for a single implant. We also accept all major insurance plans that cover implants.</div>
                            </div>
                            <div class="faq-item">
                                <div class="faq-question">Why are your prices lower than others?</div>
                                <div class="faq-answer">We believe in transparent, fair pricing. By handling everything in-house and using efficient systems, we avoid the markups common at other practices. Our £1,999 complete package price includes everything - no hidden fees.</div>
                            </div>
                        </div>
                    </div>
                </section>
                
                <section class="cta-section">
                    <div class="container">
                        <h2>Start Your Journey to a Complete Smile</h2>
                        <p>Book your consultation today and discover how implants can transform your life</p>
                        <div class="cta-buttons">
                            <a href="{BOOKING_LINK}" class="book-now">Book Consultation</a>
                            <a href="tel:01234567890" class="call-now">Call Now</a>
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
        
    );
    
    css(site);
    
    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("implants", r##"
    
    {}
        
        main.implants {
        
        
        
            .dentist-splash-hero {
                .price {
                
                    font-weight: 600;
                    font-size: 26px;
                
                }
            }
            
            
            /* Key Benefits */
            .benefits {
                padding: 80px var(--site-padding-x);
                background: #f8f9fa;
                z-index: 10;
                position: relative;
                h2 {
                    text-align: center;
                    font-size: 36px;
                    margin-bottom: 50px;
                    color: #2c3e50;
                }
                .benefits-grid {
                    display: grid;
                    grid-template-columns: repeat(3, 1fr);
                    gap: 40px;
                    max-width: 1200px;
                    margin: 0 auto;
                    .benefit-card {
                        background: white;
                        padding: 30px;
                        border-radius: 10px;
                        box-shadow: 0 2px 10px rgba(0,0,0,0.08);
                        text-align: center;
                        
                        
                            @media (max-width: 768px) {
                            
                                margin-bottom: 30px;
                            }
                        .icon {
                            width: 60px;
                            height: 60px;
                            margin: 0 auto 20px;
                            background: #20b2aa;
                            border-radius: 50%;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            color: white;
                            font-size: 24px;
                        }
                        h3 {
                            font-size: 22px;
                            margin-bottom: 15px;
                            color: #2c3e50;
                        }
                        p {
                            color: #6c757d;
                            line-height: 1.6;
                        }
                    }
                    
                    @media (max-width: 768px) {
                        display: block;
                    }
                }
            }
            
            /* Process Section */
            .process {
                padding: 80px var(--site-padding-x);
                h2 {
                    text-align: center;
                    font-size: 36px;
                    margin-bottom: 20px;
                    color: #2c3e50;
                }
                .process-intro {
                    text-align: center;
                    font-size: 18px;
                    color: #6c757d;
                    margin-bottom: 50px;
                    max-width: 600px;
                    margin-left: auto;
                    margin-right: auto;
                }
                .timeline {
                    position: relative;
                    max-width: 900px;
                    margin: 0 auto;
                    &:before {
                        content: '';
                        position: absolute;
                        left: 50%;
                        top: 0;
                        bottom: 0;
                        width: 2px;
                        background: #20b2aa;
                        
                        @media (max-width: 768px) {
                            left: 20px;
                        }
                    }
                    .timeline-item {
                        padding: 20px 0;
                        position: relative;
                        width: 50%;
                        box-sizing: border-box;
                        &:nth-child(odd) {
                            left: 2px;
                            padding-right: 40px;
                            text-align: right;
                        }
                        &:nth-child(even) {
                            left: 50%;
                            padding-left: 40px;
                        }
                        @media (max-width: 768px) {
                            width: 100%;
                            left: 0 !important;
                            padding-left: 60px !important;
                            padding-right: 0 !important;
                            text-align: left !important;
                        }
                        .timeline-dot {
                            position: absolute;
                            width: 20px;
                            height: 20px;
                            background: #20b2aa;
                            border: 4px solid white;
                            border-radius: 50%;
                            top: 30px;
                            @media (max-width: 768px) {
                                left: 7px !important;
                            }
                        }
                        &:nth-child(odd) .timeline-dot {
                            right: -13px;
                        }
                        &:nth-child(even) .timeline-dot {
                            left: -13px;
                        }
                        h3 {
                            font-size: 22px;
                            color: #2c3e50;
                            margin-bottom: 10px;
                        }
                        .timeline-duration {
                            color: #20b2aa;
                            font-weight: 600;
                            margin-bottom: 10px;
                        }
                        p {
                            color: #6c757d;
                            line-height: 1.6;
                        }
                    }
                }
            }
            
            /* Pricing Section */
            .pricing {
                padding: 84px var(--site-padding-x);
                background: #f8f9fa;
                h2 {
                    text-align: center;
                    font-size: 36px;
                    margin-bottom: 50px;
                    color: #2c3e50;
                }
                .pricing-cards {
                    display: grid;
                    grid-template-columns: repeat(3, 1fr);
                    gap: 40px;
                    max-width: 1000px;
                    margin: 0 auto;
                    
                    @media (max-width: 768px) {
                        display: block;
                    }
                    .pricing-card {
                        background: white;
                        border-radius: 10px;
                        padding: 40px 30px;
                        text-align: center;
                        position: relative;
                        box-shadow: 0 2px 10px rgba(0,0,0,0.08);
                        @media (max-width: 768px) {
                            max-width: 300px;
                            margin: 0 auto 48px;
                        }
                        &.featured {
                            transform: scale(1.05);
                            box-shadow: 0 5px 20px rgba(32,178,170,0.3);
                            .badge {
                                position: absolute;
                                top: -15px;
                                left: 50%;
                                transform: translateX(-50%);
                                background: #20b2aa;
                                color: white;
                                padding: 5px 20px;
                                border-radius: 20px;
                                font-size: 14px;
                                font-weight: 600;
                            }
                        }
                        h3 {
                            font-size: 24px;
                            margin-bottom: 20px;
                            color: #2c3e50;
                        }
                        .price {
                            font-size: 36px;
                            color: #20b2aa;
                            font-weight: 700;
                            margin-bottom: 10px;
                        }
                        .price-note {
                            color: #6c757d;
                            margin-bottom: 30px;
                            font-size: 14px;
                        }
                        ul {
                            list-style: none;
                            margin-bottom: 30px;
                            li {
                                padding: 10px 0;
                                color: #6c757d;
                                &:before {
                                    content: "✓";
                                    color: #20b2aa;
                                    font-weight: bold;
                                    margin-right: 10px;
                                }
                            }
                        }
                        .book-btn {
                            background: #20b2aa;
                            color: white;
                            padding: 12px 30px;
                            border-radius: 5px;
                            text-decoration: none;
                            display: inline-block;
                            font-weight: 600;
                            transition: transform 0.3s;
                            &:hover {
                                transform: translateY(-2px);
                            }
                        }
                    }
                }
            }
            
            /* FAQ Section */
            .faq {
                padding: 80px 0;
                h2 {
                    text-align: center;
                    font-size: 36px;
                    margin-bottom: 50px;
                    color: #2c3e50;
                }
                .faq-container {
                    max-width: 800px;
                    margin: 0 auto;
                    .faq-item {
                        background: white;
                        margin-bottom: 30px;
                        border-radius: 10px;
                        box-shadow: 0 2px 10px rgba(0,0,0,0.08);
                        padding: 25px;
                        .faq-question {
                            font-size: 20px;
                            font-weight: 600;
                            color: #2c3e50;
                            margin-bottom: 15px;
                        }
                        .faq-answer {
                            color: #6c757d;
                            line-height: 1.8;
                            font-size: 16px;
                        }
                    }
                }
            }
            
            /* CTA Section */
            .cta-section {
                background: linear-gradient(135deg, #20b2aa 0%, #48d1cc 100%);
                padding: 80px var(--site-padding-x);
                text-align: center;
                color: white;
                h2 {
                    font-size: 36px;
                    margin-bottom: 20px;
                    color: white;
                }
                p {
                    font-size: 20px;
                    margin-bottom: 30px;
                    opacity: 0.95;
                }
                .cta-buttons {
                    display: flex;
                    gap: 20px;
                    justify-content: center;
                    @media (max-width: 768px) {
                        flex-direction: column;
                        max-width: 300px;
                        margin: 0 auto;
                    }
                    .book-now {
                        background: white;
                        color: #20b2aa;
                        padding: 15px 40px;
                        border-radius: 5px;
                        text-decoration: none;
                        font-weight: 600;
                        font-size: 18px;
                        transition: transform 0.3s;
                        &:hover {
                            transform: translateY(-2px);
                        }
                    }
                    .call-now {
                        background: transparent;
                        color: white;
                        padding: 15px 40px;
                        border: 2px solid white;
                        border-radius: 5px;
                        text-decoration: none;
                        font-weight: 600;
                        font-size: 18px;
                        transition: background 0.3s, color 0.3s;
                        &:hover {
                            background: white;
                            color: #20b2aa;
                        }
                    }
                }
            }
            
        }
        
    "##);
}
        