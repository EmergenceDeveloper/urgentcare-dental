use crate::prelude::*;

pub fn construct_smile_preview(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
    add_default_og_image(page);
    let head = site.construct_head(page);
    let header = construct_header(site, &page.foundation);
    let footer = construct_footer(site);

    css(site);

    let html = format!(
        r##"
        <!DOCTYPE html>
        <html lang="en-GB">
        {head}
        <body>
            {header}
            <main class="smile-preview">
                <section class="intro-section">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="inner">
                        <div class="text-area">
                            <h1>Preview Your Results</h1>
                            <p>See what straighter, whiter teeth could look like on you - before you book.</p>
                            <p class="treatments-note">Whether you're considering straightening, composite bonding, or implants for missing teeth - preview the end of your smile journey.</p>
                        </div>
                    </div>
                </section>
                
                <section class="tool-section">
                    <div class="inner">
                        <form hx-post="https://smile-preview.info-urgentcaredental.workers.dev"
                              hx-target="#result" 
                              hx-encoding="multipart/form-data"
                              hx-indicator="#loading">
                            
                            <div class="upload-area" id="upload-area">
                                <label for="selfie">
                                    <span class="upload-icon">📸</span>
                                    <span class="upload-text">Choose a photo or take a selfie</span>
                                    <span class="upload-hint">For best results, smile with teeth showing</span>
                                </label>
                                <input type="file" 
                                       id="selfie" 
                                       name="selfie" 
                                       accept="image/*" 
                                       capture="user" 
                                       required />
                            </div>
                            
                            <button type="submit" class="btn-primary">
                                Preview My Smile
                            </button>
                        </form>
                        
                        <div id="loading" class="htmx-indicator">
                            <div class="loading-spinner"></div>
                            <p class="loading-text">Creating your preview...</p>
                        </div>
                        
                        <div id="result"></div>
                    </div>
                </section>
                
                <section class="info-section">
                    <div class="inner">
                        <h2>How It Works</h2>
                        <div class="steps">
                            <div class="step">
                                <span class="step-number">1</span>
                                <h3>Upload a Selfie</h3>
                                <p>Take a photo showing your smile, or upload an existing one.</p>
                            </div>
                            <div class="step">
                                <span class="step-number">2</span>
                                <h3>We Create Your Preview</h3>
                                <p>Your photo is analysed and we generate a preview with aligned teeth, composite bonding, and implants for any missing teeth.</p>
                            </div>
                            <div class="step">
                                <span class="step-number">3</span>
                                <h3>See the Difference</h3>
                                <p>Compare before and after to visualise your potential transformation.</p>
                            </div>
                        </div>
                        
                        <div class="cta-box">
                            <p>Ready to make it real? Book a consultation to discuss your options.</p>
                            <a href="{BOOKING_LINK}" class="btn-primary">Book Consultation</a>
                        </div>
                    </div>
                </section>
            </main>
            {footer}
            <script src="https://unpkg.com/htmx.org@1.9.10"></script>
        </body>
        </html> 
    "##
    );

    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("smile-preview", r##"
        main.smile-preview {
            background: #f8f9fa;
            
            .intro-section {
                position: relative;
                overflow: hidden;
                
                .polka-dots {
                    position: absolute;
                    width: 100%;
                    height: 100%;
                    background-color: transparent;
                    background-image: radial-gradient(rgb(2, 220, 227) 1px, transparent 1px), radial-gradient(rgb(2, 220, 227) 1px, rgba(35, 84, 84, 0) 1px);
                    background-position: 0px 0px, 20px 20px;
                    background-size: 40px 40px;
                }
                
                .background-fade {
                    position: absolute;
                    z-index: 1;
                    width: 100%;
                    height: 100%;
                    background: linear-gradient(150deg, rgba(255, 255, 255, 0) 0%, rgb(255, 255, 255) 70%);
                }
                
                .inner {
                    max-width: 800px;
                    margin: 0 auto;
                    position: relative;
                    z-index: 10;
                    padding: 180px var(--site-padding-x) 48px;
                }
                
                .text-area {
                    text-align: center;
                    
                    h1 {
                        font-size: 48tem;
                        margin-bottom: 1.5rem;
                        font-weight: 700;
                    }
                    
                    p {
                        font-size: 18tem;
                        line-height: 1.8;
                        color: var(--grey-50);
                    }
                    
                    .treatments-note {
                        font-size: 16tem;
                        color: var(--grey-40);
                        margin-top: 0.5rem;
                    }
                }
            }
            
            .tool-section {
                padding: 60px 20px;
                
                .inner {
                    max-width: 600px;
                    margin: 0 auto;
                    text-align: center;
                }
                
                form {
                    margin-bottom: 2rem;
                }
                
                .upload-area {
                    border: 2px dashed #ccc;
                    border-radius: 12px;
                    padding: 50px 40px;
                    margin-bottom: 1.5rem;
                    cursor: pointer;
                    transition: border-color 0.3s, background 0.3s;
                    background: white;
                    
                    &:hover {
                        border-color: #0a6b6d;
                        background: #f0fafb;
                    }
                    
                    label {
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        gap: 0.5rem;
                        cursor: pointer;
                    }
                    
                    .upload-icon {
                        font-size: 3rem;
                    }
                    
                    .upload-text {
                        font-size: 1.1rem;
                        color: #333;
                        font-weight: 500;
                    }
                    
                    .upload-hint {
                        font-size: 0.9rem;
                        color: #888;
                    }
                    
                    input[type="file"] {
                        display: none;
                    }
                }
                
                .btn-primary {
                    display: inline-block;
                    padding: 14px 50px;
                    background: #0a6b6d;
                    color: white;
                    border: none;
                    border-radius: 6px;
                    font-weight: 600;
                    font-size: 1.1rem;
                    cursor: pointer;
                    transition: background 0.3s;
                    
                    &:hover {
                        background: #085456;
                    }
                }
                
                .htmx-indicator {
                    display: none;
                    padding: 40px;
                }
                
                .htmx-request .htmx-indicator {
                    display: block;
                }
                
                .htmx-request form {
                    opacity: 0.5;
                    pointer-events: none;
                }
                
                .loading-spinner {
                    width: 50px;
                    height: 50px;
                    border: 4px solid #e5e7eb;
                    border-top-color: #0a6b6d;
                    border-radius: 50%;
                    animation: spin 1s linear infinite;
                    margin: 0 auto 1rem;
                }
                
                @keyframes spin {
                    to { transform: rotate(360deg); }
                }
                
                .loading-text {
                    color: #0a6b6d;
                    font-size: 1.1rem;
                }
                
                #result {
                    
                    .smile-result {
                        margin-top: 2rem;
                        background: white;
                        padding: 30px;
                        border-radius: 12px;
                        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.08);
                        
                        .comparison {
                            display: grid;
                            grid-template-columns: 1fr 1fr;
                            gap: 20px;
                            
                            @media (max-width: 500px) {
                                grid-template-columns: 1fr;
                            }
                        }
                        
                        .before, .after {
                            
                            h3 {
                                margin-bottom: 0.75rem;
                                color: #333;
                                font-size: 1.1rem;
                            }
                            
                            img {
                                width: 100%;
                                border-radius: 8px;
                                box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                            }
                        }
                        
                        .disclaimer {
                            margin-top: 1.5rem;
                            font-size: 0.875rem;
                            color: #999;
                            font-style: italic;
                        }
                    }
                }
            }
            
            .info-section {
                padding: 80px 20px;
                background: white;
                
                .inner {
                    max-width: 900px;
                    margin: 0 auto;
                }
                
                h2 {
                    text-align: center;
                    color: #0a6b6d;
                    font-size: 2rem;
                    margin-bottom: 3rem;
                }
                
                .steps {
                    display: grid;
                    grid-template-columns: repeat(3, 1fr);
                    gap: 40px;
                    margin-bottom: 3rem;
                    
                    @media (max-width: 700px) {
                        grid-template-columns: 1fr;
                        gap: 30px;
                    }
                }
                
                .step {
                    text-align: center;
                    
                    .step-number {
                        display: inline-flex;
                        align-items: center;
                        justify-content: center;
                        width: 50px;
                        height: 50px;
                        background: #0a6b6d;
                        color: white;
                        border-radius: 50%;
                        font-size: 1.5rem;
                        font-weight: 700;
                        margin-bottom: 1rem;
                    }
                    
                    h3 {
                        color: #333;
                        margin-bottom: 0.5rem;
                        font-size: 1.2rem;
                    }
                    
                    p {
                        color: #666;
                        font-size: 0.95rem;
                        line-height: 1.6;
                    }
                }
                
                .cta-box {
                    text-align: center;
                    background: #f0fafb;
                    padding: 40px;
                    border-radius: 12px;
                    
                    p {
                        font-size: 1.1rem;
                        color: #333;
                        margin-bottom: 1.5rem;
                    }
                    
                    .btn-primary {
                        display: inline-block;
                        padding: 14px 40px;
                        background: #0a6b6d;
                        color: white;
                        text-decoration: none;
                        border-radius: 6px;
                        font-weight: 600;
                        transition: background 0.3s;
                        
                        &:hover {
                            background: #085456;
                        }
                    }
                }
            }
            
            @media (max-width: 768px) {
                .intro-section .text-area h1 {
                    font-size: 32tem;
                }
            }
        }
    "##);
}
