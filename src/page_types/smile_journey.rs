use crate::prelude::*;

pub fn construct_smile_journey(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {
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
            <main class="smile-journey">
                <section class="intro-section">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="inner">
                        <div class="text-area">
                            <p class="eyebrow">Your Smile Journey</p>
                            <h1>From First Visit to <span>Forever Smile</span></h1>
                            <p>Every smile transformation is unique. Whether you're after a quick refresh or a complete makeover, here's how your journey unfolds.</p>
                        </div>
                    </div>
                </section>
                
                <section class="journey-map">
                    <div class="inner">
                        
                        <!-- Stage 1: Foundation -->
                        <div class="stage stage-1" data-stage="1">
                            <div class="stage-marker">
                                <span class="stage-number">1</span>
                                <span class="stage-label">Foundation</span>
                            </div>
                            <div class="node node-main node-consult">
                                <div class="node-icon">🦷</div>
                                <h3>Free Consultation</h3>
                                <p>Checkup + 3D Scan + Tailored Treatment Plan</p>
                                <div class="node-glow"></div>
                            </div>
                            <div class="connector connector-down"></div>
                        </div>
                        
                        <!-- Stage 2: Health First -->
                        <div class="stage stage-2" data-stage="2">
                            <div class="stage-marker">
                                <span class="stage-number">2</span>
                                <span class="stage-label">Health First</span>
                            </div>
                            <div class="node node-main node-gum">
                                <div class="node-icon">✨</div>
                                <h3>Gum Health Spa</h3>
                                <p>Deep cleaning and gum therapy - the foundation for everything that follows</p>
                                <div class="node-glow"></div>
                            </div>
                            <div class="connector connector-split">
                                <div class="split-line left"></div>
                                <div class="split-line right"></div>
                            </div>
                        </div>
                        
                        <!-- Stage 3: Structure -->
                        <div class="stage stage-3" data-stage="3">
                            <div class="stage-marker">
                                <span class="stage-number">3</span>
                                <span class="stage-label">Structure</span>
                            </div>
                            <div class="branch-container">
                                <div class="node node-branch node-implants">
                                    <div class="node-icon">🔩</div>
                                    <h3>Implants</h3>
                                    <p>Replace missing teeth with permanent titanium roots</p>
                                    <a href="/dental-implants/" class="node-link">Learn more →</a>
                                    <div class="node-glow"></div>
                                </div>
                                <div class="branch-or">or</div>
                                <div class="node node-branch node-aligners">
                                    <div class="node-icon">😁</div>
                                    <h3>Aligners</h3>
                                    <p>Straighten your teeth invisibly over 6-18 months</p>
                                    <a href="/teeth-straightening/" class="node-link">Learn more →</a>
                                    <div class="node-glow"></div>
                                </div>
                            </div>
                            <div class="connector connector-merge">
                                <div class="merge-line left"></div>
                                <div class="merge-line right"></div>
                            </div>
                        </div>
                        
                        <!-- Stage 4: Brightness -->
                        <div class="stage stage-4" data-stage="4">
                            <div class="stage-marker">
                                <span class="stage-number">4</span>
                                <span class="stage-label">Brightness</span>
                            </div>
                            <div class="node node-main node-whitening">
                                <div class="node-icon">💎</div>
                                <h3>Whitening</h3>
                                <p>Professional-grade brightening for that Hollywood glow</p>
                                <div class="complete-badge">Many patients stop here!</div>
                                <div class="node-glow"></div>
                            </div>
                            <div class="connector connector-split">
                                <div class="split-line left"></div>
                                <div class="split-line right"></div>
                            </div>
                        </div>
                        
                        <!-- Stage 5: Perfection -->
                        <div class="stage stage-5" data-stage="5">
                            <div class="stage-marker">
                                <span class="stage-number">5</span>
                                <span class="stage-label">Perfection</span>
                            </div>
                            <div class="branch-container">
                                <div class="node node-branch node-bonding">
                                    <div class="node-icon">🎨</div>
                                    <h3>Composite Bonding</h3>
                                    <p>Reshape and refine with tooth-coloured resin</p>
                                    <a href="/composite-bonding/" class="node-link">Learn more →</a>
                                    <div class="node-glow"></div>
                                </div>
                                <div class="branch-or">or</div>
                                <div class="node node-branch node-veneers">
                                    <div class="node-icon">👑</div>
                                    <h3>Porcelain Veneers</h3>
                                    <p>The ultimate in smile transformation - custom ceramic shells</p>
                                    <a href="/porcelain-veneers/" class="node-link">Learn more →</a>
                                    <div class="node-glow"></div>
                                </div>
                            </div>
                            <div class="connector connector-down final"></div>
                        </div>
                        
                        <!-- Final Destination -->
                        <div class="stage stage-final" data-stage="final">
                            <div class="node node-destination">
                                <div class="destination-sparkles">
                                    <span class="sparkle s1">✨</span>
                                    <span class="sparkle s2">✨</span>
                                    <span class="sparkle s3">✨</span>
                                </div>
                                <div class="node-icon">🌟</div>
                                <h3>Your Dream Smile</h3>
                                <p>Confidence that lasts a lifetime</p>
                            </div>
                        </div>
                        
                    </div>
                </section>
                
                <section class="cta-section">
                    <div class="inner">
                        <div class="cta-card">
                            <div class="cta-content">
                                <h2>Ready to Start Your Journey?</h2>
                                <p>Book a free consultation and we'll create your personalised treatment plan together.</p>
                                <a href="{BOOKING_LINK}" class="btn-primary">Book Free Consultation</a>
                            </div>
                        </div>
                    </div>
                </section>
                
                <section class="preview-section">
                    <div class="inner">
                        <div class="text-area">
                            <h2>See Your Future Smile</h2>
                            <p>Upload a selfie and our AI will show you what's possible.</p>
                        </div>
                        <div class="preview-embed">
                            <div id="tool-content">
                                <form hx-post="https://smile-preview.info-urgentcaredental.workers.dev"
                                      hx-target="#tool-content"
                                      hx-swap="innerHTML"
                                      hx-encoding="multipart/form-data">
                                    
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
                                        Preview My Smile ✨
                                    </button>
                                </form>
                                
                                <div id="loading">
                                    <div class="loading-spinner"></div>
                                    <p class="loading-text">Creating your preview... this takes about 15-20 seconds</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
                
            </main>
            {footer}
            <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            <script>
                // Intersection Observer for scroll animations
                const observer = new IntersectionObserver((entries) => {{
                    entries.forEach(entry => {{
                        if (entry.isIntersecting) {{
                            entry.target.classList.add('visible');
                        }}
                    }});
                }}, {{ threshold: 0.2, rootMargin: '0px 0px -50px 0px' }});
                
                document.querySelectorAll('.stage').forEach(stage => {{
                    observer.observe(stage);
                }});
                
                // Photo upload handling
                document.getElementById('selfie').addEventListener('change', function(e) {{
                    const file = e.target.files[0];
                    if (file) {{
                        const reader = new FileReader();
                        reader.onload = function(event) {{
                            const uploadArea = document.getElementById('upload-area');
                            uploadArea.style.borderStyle = 'solid';
                            uploadArea.style.borderColor = '#0a6b6d';
                            uploadArea.style.background = '#f0fafb';
                            
                            const existingPreview = uploadArea.querySelector('.preview-img');
                            if (existingPreview) existingPreview.remove();
                            
                            const img = document.createElement('img');
                            img.src = event.target.result;
                            img.className = 'preview-img';
                            img.style.cssText = 'max-width: 100%; max-height: 300px; border-radius: 8px; margin-top: 1rem; pointer-events: none;';
                            uploadArea.appendChild(img);
                            
                            uploadArea.querySelector('.upload-icon').textContent = '✓';
                            uploadArea.querySelector('.upload-icon').style.color = '#0a6b6d';
                            uploadArea.querySelector('.upload-text').textContent = 'Ready! Click below to preview your smile.';
                            uploadArea.querySelector('.upload-hint').style.display = 'none';
                            
                            const btn = document.querySelector('.preview-section .btn-primary');
                            setTimeout(() => {{
                                btn.scrollIntoView({{ behavior: 'smooth', block: 'center' }});
                                setTimeout(() => btn.classList.add('glow-pulse'), 450);
                            }}, 500);
                        }};
                        reader.readAsDataURL(file);
                    }}
                }});

                document.body.addEventListener('htmx:afterSwap', function(e) {{
                    if (e.detail.target.id === 'tool-content') {{
                        setTimeout(() => {{
                            const result = document.querySelector('.smile-result');
                            if (result) {{
                                result.scrollIntoView({{ behavior: 'smooth', block: 'center' }});
                            }}
                        }}, 100);
                    }}
                }});
            </script>
        </body>
        </html> 
    "##
    );

    page.foundation.content = Some(html);
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("smile-journey", r##"

        @keyframes float {
            0%, 100% { transform: translateY(0); }
            50% { transform: translateY(-10px); }
        }
        
        @keyframes pulse-glow {
            0%, 100% { opacity: 0.5; transform: scale(1); }
            50% { opacity: 0.8; transform: scale(1.1); }
        }
        
        @keyframes sparkle-float {
            0% { opacity: 0; transform: translateY(0) scale(0); }
            50% { opacity: 1; transform: translateY(-20px) scale(1); }
            100% { opacity: 0; transform: translateY(-40px) scale(0); }
        }
        
        @keyframes draw-line {
            from { height: 0; }
            to { height: 60px; }
        }
        
        @keyframes slide-up {
            from { opacity: 0; transform: translateY(40px); }
            to { opacity: 1; transform: translateY(0); }
        }
        
        @keyframes spin {
            to { transform: rotate(360deg); }
        }

        @keyframes glow-pulse {
            0% { box-shadow: 0 0 0 0 rgba(104, 248, 253, 0.8); }
            70% { box-shadow: 0 0 0 15px rgba(104, 248, 253, 0); }
            100% { box-shadow: 0 0 0 0 rgba(104, 248, 253, 0); }
        }

        main.smile-journey {
            background: linear-gradient(180deg, #f8f9fa 0%, #fff 100%);
            
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
                    padding: 180px var(--site-padding-x) 60px;
                }
                
                .text-area {
                    text-align: center;
                    
                    .eyebrow {
                        font-size: 14tem;
                        text-transform: uppercase;
                        letter-spacing: 2px;
                        color: var(--turquoise-30);
                        margin-bottom: 16px;
                        font-weight: 600;
                    }
                    
                    h1 {
                        font-size: 48tem;
                        margin-bottom: 1.5rem;
                        font-weight: 700;
                        line-height: 1.1;

                        span {
                            color: var(--turquoise-30);
                            font-style: italic;
                        }
                    }
                    
                    p {
                        font-size: 18tem;
                        line-height: 1.8;
                        color: var(--grey-50);
                    }
                }
            }
            
            .journey-map {
                position: relative;
                
                .inner {
                    max-width: 900px;
                    margin: 0 auto;
                    padding: 40px var(--site-padding-x) 80px;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }
                
                .stage {
                    width: 100%;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    opacity: 0;
                    transform: translateY(40px);
                    transition: all 0.6s cubic-bezier(0.22, 1, 0.36, 1);
                    
                    &.visible {
                        opacity: 1;
                        transform: translateY(0);
                    }
                    
                    &[data-stage="2"].visible { transition-delay: 0.1s; }
                    &[data-stage="3"].visible { transition-delay: 0.2s; }
                    &[data-stage="4"].visible { transition-delay: 0.3s; }
                    &[data-stage="5"].visible { transition-delay: 0.4s; }
                    &[data-stage="final"].visible { transition-delay: 0.5s; }
                }
                
                .stage-marker {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    margin-bottom: 20px;
                    
                    .stage-number {
                        width: 32px;
                        height: 32px;
                        background: var(--turquoise-30);
                        color: white;
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-weight: 700;
                        font-size: 14tem;
                    }
                    
                    .stage-label {
                        font-size: 12tem;
                        text-transform: uppercase;
                        letter-spacing: 2px;
                        color: var(--grey-50);
                        font-weight: 600;
                    }
                }
                
                .node {
                    position: relative;
                    background: white;
                    border-radius: 20px;
                    padding: 32px;
                    text-align: center;
                    box-shadow: 0 4px 24px rgba(0, 111, 115, 0.08);
                    border: 2px solid transparent;
                    transition: all 0.3s ease;
                    
                    &:hover {
                        transform: translateY(-4px);
                        box-shadow: 0 8px 32px rgba(0, 111, 115, 0.15);
                        border-color: var(--turquoise-70);
                        
                        .node-glow {
                            opacity: 1;
                        }
                    }
                    
                    .node-icon {
                        font-size: 48px;
                        margin-bottom: 16px;
                        animation: float 3s ease-in-out infinite;
                    }
                    
                    h3 {
                        font-size: 24tem;
                        color: var(--turquoise-15);
                        margin-bottom: 8px;
                    }
                    
                    p {
                        font-size: 16tem;
                        color: var(--grey-50);
                        line-height: 1.6;
                        margin: 0;
                    }
                    
                    .node-link {
                        display: inline-block;
                        margin-top: 12px;
                        font-size: 14tem;
                        color: var(--turquoise-30);
                        font-weight: 600;
                        text-decoration: none;
                        
                        &:hover {
                            color: var(--turquoise-15);
                        }
                    }
                    
                    .node-glow {
                        position: absolute;
                        inset: -2px;
                        border-radius: 22px;
                        background: linear-gradient(135deg, var(--turquoise-70), var(--turquoise-30));
                        opacity: 0;
                        z-index: -1;
                        filter: blur(12px);
                        transition: opacity 0.3s ease;
                    }
                    
                    .complete-badge {
                        display: inline-block;
                        margin-top: 16px;
                        padding: 8px 16px;
                        background: var(--turquoise-90);
                        color: var(--turquoise-15);
                        border-radius: 20px;
                        font-size: 13tem;
                        font-weight: 600;
                    }
                }
                
                .node-main {
                    width: 100%;
                    max-width: 480px;
                }
                
                .branch-container {
                    display: flex;
                    gap: 24px;
                    align-items: stretch;
                    width: 100%;
                    max-width: 700px;
                    
                    .branch-or {
                        display: flex;
                        align-items: center;
                        font-size: 14tem;
                        color: var(--grey-50);
                        font-weight: 600;
                        text-transform: uppercase;
                        letter-spacing: 1px;
                    }
                    
                    .node-branch {
                        flex: 1;
                    }
                }
                
                .connector {
                    width: 4px;
                    height: 60px;
                    background: linear-gradient(180deg, var(--turquoise-30), var(--turquoise-70));
                    border-radius: 2px;
                    margin: 20px 0;
                    position: relative;
                }
                
                .connector-split {
                    width: 100%;
                    max-width: 500px;
                    height: 60px;
                    background: transparent;
                    position: relative;
                    
                    &::before {
                        content: '';
                        position: absolute;
                        top: 0;
                        left: 50%;
                        width: 4px;
                        height: 30px;
                        background: linear-gradient(180deg, var(--turquoise-30), var(--turquoise-70));
                        transform: translateX(-50%);
                        border-radius: 2px;
                    }
                    
                    .split-line {
                        position: absolute;
                        top: 28px;
                        height: 4px;
                        width: calc(50% - 2px);
                        border-radius: 2px;
                        
                        &.left {
                            left: 0;
                            background: linear-gradient(90deg, var(--turquoise-70), var(--turquoise-30));
                            border-radius: 2px 0 0 2px;
                            
                            &::after {
                                content: '';
                                position: absolute;
                                left: 0;
                                top: 0;
                                width: 4px;
                                height: 28px;
                                background: var(--turquoise-70);
                                border-radius: 0 0 2px 2px;
                            }
                        }
                        
                        &.right {
                            right: 0;
                            background: linear-gradient(90deg, var(--turquoise-30), var(--turquoise-70));
                            border-radius: 0 2px 2px 0;
                            
                            &::after {
                                content: '';
                                position: absolute;
                                right: 0;
                                top: 0;
                                width: 4px;
                                height: 28px;
                                background: var(--turquoise-70);
                                border-radius: 0 0 2px 2px;
                            }
                        }
                    }
                }
                
                .connector-merge {
                    width: 100%;
                    max-width: 500px;
                    height: 60px;
                    background: transparent;
                    position: relative;
                    
                    &::after {
                        content: '';
                        position: absolute;
                        bottom: 0;
                        left: 50%;
                        width: 4px;
                        height: 30px;
                        background: linear-gradient(180deg, var(--turquoise-70), var(--turquoise-30));
                        transform: translateX(-50%);
                        border-radius: 2px;
                    }
                    
                    .merge-line {
                        position: absolute;
                        bottom: 28px;
                        height: 4px;
                        width: calc(50% - 2px);
                        border-radius: 2px;
                        
                        &.left {
                            left: 0;
                            background: linear-gradient(90deg, var(--turquoise-70), var(--turquoise-30));
                            
                            &::before {
                                content: '';
                                position: absolute;
                                left: 0;
                                bottom: 0;
                                width: 4px;
                                height: 28px;
                                background: var(--turquoise-70);
                                border-radius: 2px 2px 0 0;
                            }
                        }
                        
                        &.right {
                            right: 0;
                            background: linear-gradient(90deg, var(--turquoise-30), var(--turquoise-70));
                            
                            &::before {
                                content: '';
                                position: absolute;
                                right: 0;
                                bottom: 0;
                                width: 4px;
                                height: 28px;
                                background: var(--turquoise-70);
                                border-radius: 2px 2px 0 0;
                            }
                        }
                    }
                }
                
                .connector.final {
                    height: 80px;
                }
                
                .node-destination {
                    position: relative;
                    background: linear-gradient(135deg, var(--turquoise-15), var(--turquoise-30));
                    color: white;
                    padding: 48px 64px;
                    border-radius: 24px;
                    
                    h3 {
                        color: white;
                        font-size: 28tem;
                    }
                    
                    p {
                        color: var(--turquoise-90);
                    }
                    
                    .destination-sparkles {
                        position: absolute;
                        inset: 0;
                        pointer-events: none;
                        
                        .sparkle {
                            position: absolute;
                            font-size: 24px;
                            animation: sparkle-float 2s ease-in-out infinite;
                            
                            &.s1 { top: 10%; left: 10%; animation-delay: 0s; }
                            &.s2 { top: 20%; right: 15%; animation-delay: 0.6s; }
                            &.s3 { bottom: 20%; left: 20%; animation-delay: 1.2s; }
                        }
                    }
                }
            }
            
            .cta-section {
                .inner {
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 0 var(--site-padding-x) 80px;
                }
                
                .cta-card {
                    background: var(--turquoise-98);
                    border-radius: 24px;
                    padding: 48px;
                    text-align: center;
                    border: 2px solid var(--turquoise-70);
                    
                    h2 {
                        font-size: 32tem;
                        margin-bottom: 16px;
                    }
                    
                    p {
                        font-size: 18tem;
                        color: var(--grey-50);
                        margin-bottom: 24px;
                    }
                    
                    .btn-primary {
                        display: inline-block;
                        padding: 16px 32px;
                        background: var(--turquoise-15);
                        color: white;
                        border-radius: 8px;
                        font-weight: 600;
                        font-size: 16tem;
                        text-decoration: none;
                        transition: all 0.3s ease;
                        
                        &:hover {
                            background: var(--turquoise-30);
                            transform: translateY(-2px);
                        }
                    }
                }
            }
            
            .preview-section {
                background: var(--turquoise-15);
                
                .inner {
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 80px var(--site-padding-x);
                }
                
                .text-area {
                    text-align: center;
                    margin-bottom: 40px;
                    
                    h2 {
                        color: white;
                        font-size: 36tem;
                        margin-bottom: 12px;
                    }
                    
                    p {
                        color: var(--turquoise-70);
                        font-size: 18tem;
                    }
                }
                
                .preview-embed {
                    background: white;
                    border-radius: 16px;
                    padding: 32px;
                    
                    form {
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        gap: 24px;
                    }
                    
                    .upload-area {
                        width: 100%;
                        padding: 40px;
                        border: 3px dashed var(--turquoise-70);
                        border-radius: 12px;
                        background: var(--turquoise-98);
                        cursor: pointer;
                        transition: all 0.3s ease;
                        text-align: center;
                        
                        &:hover {
                            border-color: var(--turquoise-30);
                            background: #e8f8f9;
                        }
                        
                        label {
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            gap: 8px;
                            cursor: pointer;
                        }
                        
                        input[type="file"] {
                            display: none;
                        }
                        
                        .upload-icon {
                            font-size: 48px;
                        }
                        
                        .upload-text {
                            font-size: 18tem;
                            font-weight: 600;
                            color: var(--turquoise-15);
                        }
                        
                        .upload-hint {
                            font-size: 14tem;
                            color: var(--grey-50);
                        }
                    }
                    
                    .btn-primary {
                        display: inline-block;
                        padding: 16px 32px;
                        background: var(--turquoise-15);
                        color: white;
                        border-radius: 8px;
                        font-weight: 600;
                        font-size: 16tem;
                        text-decoration: none;
                        border: none;
                        cursor: pointer;
                        transition: all 0.3s ease;
                        
                        &:hover {
                            background: var(--turquoise-30);
                            transform: translateY(-2px);
                        }
                        
                        &.glow-pulse {
                            animation: glow-pulse 0.8s ease-out;
                        }
                    }
                    
                    #loading {
                        display: none;
                        padding: 40px;
                        text-align: center;
                    }
                    
                    form.htmx-request ~ #loading {
                        display: block;
                    }
                    
                    form.htmx-request {
                        opacity: 0.5;
                        pointer-events: none;
                    }
                    
                    .loading-spinner {
                        width: 50px;
                        height: 50px;
                        border: 4px solid #e5e7eb;
                        border-top-color: var(--turquoise-30);
                        border-radius: 50%;
                        animation: spin 1s linear infinite;
                        margin: 0 auto 1rem;
                    }
                    
                    .loading-text {
                        color: var(--turquoise-15);
                        font-size: 16tem;
                    }
                }
            }
            
            @media screen and (max-width: 768px) {
                .intro-section {
                    .inner {
                        padding: 140px var(--site-padding-x) 40px;
                    }
                    
                    .text-area h1 {
                        font-size: 32tem;
                    }
                }
                
                .journey-map {
                    .branch-container {
                        flex-direction: column;
                        max-width: 100%;
                        
                        .branch-or {
                            justify-content: center;
                            padding: 12px 0;
                        }
                    }
                    
                    .connector-split,
                    .connector-merge {
                        display: none;
                    }
                    
                    .connector {
                        display: block;
                    }
                    
                    .node {
                        padding: 24px;
                        
                        h3 {
                            font-size: 20tem;
                        }
                    }
                    
                    .node-destination {
                        padding: 32px 24px;
                        
                        h3 {
                            font-size: 24tem;
                        }
                    }
                }
                
                .cta-section {
                    .cta-card {
                        padding: 32px 24px;
                        
                        h2 {
                            font-size: 24tem;
                        }
                    }
                }
                
                .preview-section {
                    .text-area h2 {
                        font-size: 28tem;
                    }
                    
                    .preview-embed {
                        padding: 24px 16px;
                        
                        .upload-area {
                            padding: 24px;
                        }
                    }
                }
            }
        }
    "##);
}
