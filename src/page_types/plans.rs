use crate::prelude::*;

pub fn construct_plans(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    
    add_default_og_image(page);
    let head = site.construct_head(page);
    let header = construct_header(site, &page.foundation);
    let footer = construct_footer(site);
    
    
    css(site);
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-GB">
        {head}
        <body>
            {header}
            <main class="plans">
                <section class="intro-section">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="inner">
                        <div class="text-area">
                            <h1>Choose Your Dental Care Plan</h1>
                            <p>Affordable monthly plans designed to keep your smile healthy and bright. Whether you're maintaining already-healthy teeth, caring for your whole family, or addressing ongoing dental concerns, we have the perfect plan for you.</p>
                            <p>All plans include priority emergency appointments, professional cleanings, comprehensive exams, and free whitening top-ups. Plus, save on all additional treatments you may need.</p>
                        </div>
                    </div>
                </section>
                
                <section class="plans-grid-section">
                    <div class="inner">
                        <div class="plans-grid">
                            <div class="plan-card">
                                <h2>Protection Plan</h2>
                                <p class="description">Protecting and preserving your already-healthy smile with essential care.</p>
                                
                                <ul class="benefits">
                                    <li>Two comprehensive dental exams annually</li>
                                    <li>Two professional cleaning appointments per year</li>
                                    <li>One emergency dental consultation per year</li>
                                    <li>Priority for same-day emergency bookings</li>
                                    <li>10% savings on all additional treatments</li>
                                    <li>Free whitening top-up (2 syringes per year)</li>
                                    <li>Free skin consultations</li>
                                    <li>Free hair transplant consultations</li>
                                </ul>
                                
                                <div class="ideal-for">
                                    <strong>Ideal for:</strong> Maintaining already healthy teeth
                                </div>
                                
                                <div class="price-section">
                                    <div class="price-label">Price</div>
                                    <div class="price">£19.99</div>
                                    <div class="price-period">per month</div>
                                    <span class="whitening-note">FREE WHITENING INCLUDED</span>
                                    <a href="https://buy.stripe.com/6oE28udMxbxpdoceUV" class="btn-primary">Buy Now</a>
                                </div>
                            </div>
                            
                            <div class="plan-card featured">
                                <span class="featured-badge">BEST VALUE FOR FAMILIES</span>
                                <h2>Family Plan</h2>
                                <p class="description">Complete dental care for the whole family at an unbeatable price.</p>
                                
                                <ul class="benefits">
                                    <li>Two comprehensive dental exams annually per family member</li>
                                    <li>Two professional cleaning appointments per year per family member</li>
                                    <li>One emergency dental consultation per year for the family</li>
                                    <li>Priority for same-day emergency bookings</li>
                                    <li>10% savings on all additional treatments</li>
                                    <li>Free whitening top-up (2 syringes per year for adults)</li>
                                    <li>Free skin consultations</li>
                                    <li>Free hair transplant consultations</li>
                                </ul>
                                
                                <div class="ideal-for">
                                    <strong>Ideal for:</strong> Families seeking comprehensive dental care
                                </div>
                                
                                <div class="family-options">
                                    <label for="children-select">Add children under 16:</label>
                                    <select id="children-select" onchange="updateFamilyPrice()">
                                        <option value="0">No additional children</option>
                                        <option value="1">1 child (+£15/month)</option>
                                        <option value="2">2 children (+£30/month)</option>
                                        <option value="3">3 children (+£45/month)</option>
                                        <option value="4">4 children (+£60/month)</option>
                                        <option value="5">5 children (+£75/month)</option>
                                    </select>
                                    <div class="additional-cost" id="family-total">Base price for 2 adults</div>
                                </div>
                                
                                <div class="price-section">
                                    <div class="price-label">Price</div>
                                    <div class="price" id="family-price">£35.00</div>
                                    <div class="price-period">per month</div>
                                    <span class="whitening-note">FREE WHITENING INCLUDED</span>
                                    <a href="https://buy.stripe.com/cNi5kF9g6bEM6A17a1g7e0m" class="btn-primary">Buy Now</a>
                                </div>
                            </div>
                            
                            <div class="plan-card">
                                <h2>Proactive Recovery Plan</h2>
                                <p class="description">Improving and restoring your dental health with enhanced intervention.</p>
                                
                                <ul class="benefits">
                                    <li>Two comprehensive dental exams annually</li>
                                    <li>Four professional cleaning appointments per year (quarterly)</li>
                                    <li>One emergency dental consultation per year</li>
                                    <li>Priority for same-day emergency bookings</li>
                                    <li>Enhanced 15% savings on all additional treatments</li>
                                    <li>Free whitening top-up (2 syringes per year)</li>
                                    <li>Free skin consultations</li>
                                    <li>Free hair transplant consultations</li>
                                </ul>
                                
                                <div class="ideal-for">
                                    <strong>Ideal for:</strong> Addressing ongoing dental concerns
                                </div>
                                
                                <div class="price-section">
                                    <div class="price-label">Price</div>
                                    <div class="price">£34.99</div>
                                    <div class="price-period">per month</div>
                                    <span class="whitening-note">FREE WHITENING INCLUDED</span>
                                    <a href="https://buy.stripe.com/cN214q37T44X0Bq002" class="btn-primary">Buy Now</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
                
                <section class="contact-section">
                    <div class="inner">
                        <h2>Need Help Choosing?</h2>
                        <p>Our team is here to help you select the perfect plan for your dental health needs.</p>
                        <a href="/contact" class="btn-primary">Contact Us</a>
                    </div>
                </section>
            </main>
            {footer}
            
            <script>
                function updateFamilyPrice() {{
                    const select = document.getElementById('children-select');
                    const priceElement = document.getElementById('family-price');
                    const totalElement = document.getElementById('family-total');
                    
                    const basePrice = 35;
                    const childPrice = 15;
                    const numberOfChildren = parseInt(select.value);
                    const totalPrice = basePrice + (numberOfChildren * childPrice);
                    
                    priceElement.textContent = `£${{totalPrice.toFixed(2)}}`;
                    
                    if (numberOfChildren === 0) {{
                        totalElement.textContent = 'Base price for 2 adults';
                    }} else if (numberOfChildren === 1) {{
                        totalElement.textContent = `Total for 2 adults and 1 child`;
                    }} else {{
                        totalElement.textContent = `Total for 2 adults and ${{numberOfChildren}} children`;
                    }}
                }}
            </script>
        </body>
        </html> 
    "##);
    
    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("plans", r##"
        main.plans {
        
            background: #f8f9fa;
            
            .intro-section {
                position: relative;
                overflow: hidden;
                display: block;
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
                    margin: 0 auto;
                    position: relative;
                    z-index: 10;
                    padding: 180px var(--site-padding-x) 48px;
                }
                
                .text-area {
                    max-width: 800px;
                    margin: 0 auto;
                    text-align: center;
                    z-index: 10;
                    
                    h1 {
                        font-size: 48tem;
                        margin-bottom: 1.5rem;
                        font-weight: 700;
                    }
                    
                    p {
                        font-size: 18tem;
                        line-height: 1.8;
                        margin-bottom: 1rem;
                        color: var(--grey-50);
                        
                        &:last-child {
                            margin-bottom: 0;
                        }
                    }
                }
            }
            
            .plans-grid-section {
                padding: 60px 20px;
                
                .inner {
                    max-width: 1200px;
                    margin: 0 auto;
                }
                
                .plans-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
                    gap: 30px;
                }
                
                .plan-card {
                    background: white;
                    border-radius: 12px;
                    padding: 50px 30px 30px;
                    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.08);
                    transition: transform 0.3s ease, box-shadow 0.3s ease;
                    position: relative;
                    
                    &:hover {
                        transform: translateY(-5px);
                        box-shadow: 0 8px 25px rgba(0, 0, 0, 0.12);
                    }
                    
                    &.featured {
                        border: 2px solid #0a6b6d;
                    }
                    
                    .featured-badge {
                        position: absolute;
                        top: -12px;
                        left: 50%;
                        transform: translateX(-50%);
                        background: #0a6b6d;
                        color: white;
                        padding: 4px 16px;
                        border-radius: 20px;
                        font-size: 0.875rem;
                        font-weight: 600;
                        text-align: center;
                        min-width: max-content;
                    }
                    
                    h2 {
                        font-size: 1.5rem;
                        font-weight: 600;
                        color: #0a6b6d;
                        margin-bottom: 0.5rem;
                    }
                    
                    .description {
                        color: #666;
                        font-size: 0.875rem;
                        margin-bottom: 1.5rem;
                        min-height: 60px;
                    }
                    
                    .benefits {
                        list-style: none;
                        margin-bottom: 1.5rem;
                        
                        li {
                            padding: 8px 0;
                            padding-left: 28px;
                            position: relative;
                            color: #555;
                            font-size: 0.95rem;
                            
                            &:before {
                                content: "✓";
                                position: absolute;
                                left: 0;
                                color: #0a6b6d;
                                font-weight: 600;
                            }
                        }
                    }
                    
                    .ideal-for {
                        background: #f0fafb;
                        padding: 10px 15px;
                        border-radius: 8px;
                        margin-bottom: 1.5rem;
                        font-size: 0.875rem;
                        
                        strong {
                            color: #0a6b6d;
                        }
                    }
                    
                    .family-options {
                        margin: 1.5rem 0;
                        padding: 15px;
                        background: #f9fafb;
                        border-radius: 8px;
                        
                        label {
                            display: block;
                            font-size: 0.875rem;
                            color: #555;
                            margin-bottom: 8px;
                        }
                        
                        select {
                            width: 100%;
                            padding: 10px;
                            border: 1px solid #ddd;
                            border-radius: 6px;
                            font-size: 0.95rem;
                            background: white;
                            cursor: pointer;
                            appearance: none;
                            background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
                            background-repeat: no-repeat;
                            background-position: right 10px center;
                            background-size: 20px;
                            padding-right: 40px;
                            box-sizing: border-box;
                        }
                        
                        .additional-cost {
                            margin-top: 8px;
                            font-size: 0.875rem;
                            color: #666;
                        }
                    }
                    
                    .price-section {
                        text-align: center;
                        padding-top: 1.5rem;
                        border-top: 1px solid #e5e7eb;
                        
                        .price-label {
                            color: #999;
                            font-size: 0.875rem;
                            margin-bottom: 0.5rem;
                        }
                        
                        .price {
                            font-size: 2rem;
                            font-weight: 700;
                            color: #0a6b6d;
                            margin-bottom: 0.25rem;
                        }
                        
                        .price-period {
                            color: #666;
                            font-size: 0.875rem;
                            margin-bottom: 1rem;
                        }
                        
                        .whitening-note {
                            display: inline-block;
                            background: #fff3cd;
                            color: #856404;
                            padding: 4px 12px;
                            border-radius: 20px;
                            font-size: 0.75rem;
                            font-weight: 600;
                            margin-bottom: 1rem;
                        }
                        
                        .btn-primary {
                            display: block;
                            width: 100%;
                            padding: 12px 24px;
                            background: #0a6b6d;
                            color: white;
                            text-decoration: none;
                            border-radius: 6px;
                            font-weight: 600;
                            text-align: center;
                            transition: background 0.3s ease;
                            border: none;
                            cursor: pointer;
                            font-size: 1rem;
                            margin-top: 1rem;
                            box-sizing: border-box;
                            
                            &:hover {
                                background: #085456;
                            }
                        }
                    }
                }
            }
            
            .contact-section {
                padding: 60px 20px;
                background: white;
                
                .inner {
                    max-width: 800px;
                    margin: 0 auto;
                    text-align: center;
                    
                    h2 {
                        color: #0a6b6d;
                        margin-bottom: 1rem;
                        font-size: 2rem;
                    }
                    
                    p {
                        color: #666;
                        margin-bottom: 1.5rem;
                        font-size: 1.125rem;
                    }
                    
                    .btn-primary {
                        display: inline-block;
                        padding: 12px 40px;
                        background: #0a6b6d;
                        color: white;
                        text-decoration: none;
                        border-radius: 6px;
                        font-weight: 600;
                        transition: background 0.3s ease;
                        font-size: 1rem;
                        
                        &:hover {
                            background: #085456;
                        }
                    }
                }
            }
            
            @media (max-width: 768px) {
                .intro-section {
                    
                    .text-area h1 {
                        font-size: 32tem;
                    }
                }
                
                .plans-grid-section .plans-grid {
                    grid-template-columns: 1fr;
                }
            }
        }
    "##);
}