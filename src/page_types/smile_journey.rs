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

                <section class="hero-section">
                    <div class="polka-dots"></div>
                    <div class="background-fade"></div>
                    <div class="inner">
                        <div class="text-area">
                            <p class="eyebrow">Build Your Smile</p>
                            <h1>What's Your <span>Dream Smile</span>?</h1>
                            <p class="subtitle">Select your goals and we'll show your personalized path</p>
                        </div>

                        <div class="goal-cards" id="goal-cards">
                            <button class="goal-card" data-goal="fill-gaps">
                                <div class="goal-icon">🦷</div>
                                <h3>Fill Gaps</h3>
                                <p>Replace missing teeth</p>
                            </button>
                            <button class="goal-card" data-goal="straighten">
                                <div class="goal-icon">😁</div>
                                <h3>Straighten</h3>
                                <p>Align crooked teeth</p>
                            </button>
                            <button class="goal-card" data-goal="brighten">
                                <div class="goal-icon">✨</div>
                                <h3>Brighten</h3>
                                <p>Whiter, brighter smile</p>
                            </button>
                            <button class="goal-card" data-goal="perfect-shape">
                                <div class="goal-icon">💎</div>
                                <h3>Perfect Shape</h3>
                                <p>Fix chips and uneven teeth</p>
                            </button>
                            <button class="goal-card" data-goal="complete">
                                <div class="goal-icon">👑</div>
                                <h3>Complete Makeover</h3>
                                <p>Full smile transformation</p>
                            </button>
                            <button class="goal-card" data-goal="exploring">
                                <div class="goal-icon">🔍</div>
                                <h3>Just Exploring</h3>
                                <p>See what's possible</p>
                            </button>
                        </div>
                    </div>
                </section>

                <section class="journey-section" id="journey-section">
                    <div class="inner">
                        <div class="journey-header">
                            <h2>Your Personalized Path</h2>
                            <p class="journey-subtitle" id="journey-subtitle">Select your goals above to build your journey</p>
                        </div>

                        <div class="journey-layout">
                            <div class="journey-map" id="journey-map">
                                <!-- Journey nodes will be rendered here by JS -->
                                <div class="empty-state" id="empty-state">
                                    <div class="empty-icon">🗺️</div>
                                    <p>Your journey map will appear here</p>
                                    <p class="hint">Select one or more goals above to get started</p>
                                </div>
                            </div>

                            <aside class="estimator" id="estimator">
                                <div class="estimator-header">
                                    <h3>Your Journey Estimate</h3>
                                </div>
                                <div class="estimator-empty" id="estimator-empty">
                                    <p>Select your smile goals above to see your personalised estimate</p>
                                </div>
                                <div class="estimator-body" id="estimator-body">
                                    <div class="estimate-row">
                                        <span class="label">Timeline</span>
                                        <span class="value" id="est-timeline">—</span>
                                    </div>
                                    <div class="estimate-row">
                                        <span class="label">Investment</span>
                                        <span class="value" id="est-cost">—</span>
                                    </div>
                                    <div class="estimate-row highlight">
                                        <span class="label">Monthly (0% finance)</span>
                                        <span class="value" id="est-monthly">—</span>
                                    </div>
                                </div>
                                <div class="estimator-actions" id="estimator-actions">
                                    <button class="btn-secondary" id="btn-save">
                                        <span class="save-icon">💾</span>
                                        Save My Journey
                                    </button>
                                    <a href="{BOOKING_LINK}" class="btn-primary">Book Free Consultation</a>
                                </div>
                                <div class="estimator-note" id="estimator-note">
                                    <p>Estimates are guidelines. Your consultation will provide an exact quote.</p>
                                </div>
                            </aside>
                        </div>
                    </div>
                </section>

                <section class="preview-section">
                    <div class="inner">
                        <div class="text-area">
                            <h2>See Your Future Smile</h2>
                            <p id="preview-subtitle">One selfie, and you'll see what's possible</p>
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

                <section class="cta-section">
                    <div class="inner">
                        <div class="cta-card">
                            <div class="cta-content">
                                <h2 id="cta-title">Ready to Start Your Journey?</h2>
                                <p id="cta-subtitle">Book a free consultation and we'll create your personalised treatment plan together.</p>
                                <div class="cta-buttons">
                                    <a href="{BOOKING_LINK}" class="btn-primary">Book Free Consultation</a>
                                    <button class="btn-secondary" id="btn-share">
                                        <span>🔗</span> Share My Journey
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

            </main>
            {footer}
            <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            <script>
            {script}
            </script>
        </body>
        </html>
    "##,
        script = journey_script()
    );

    page.foundation.content = Some(html);
}

fn journey_script() -> &'static str {
    r##"
    // ═══════════════════════════════════════════════════════════
    // SMILE JOURNEY - Interactive Journey Builder
    // ═══════════════════════════════════════════════════════════

    const TREATMENTS = {
        consultation: {
            id: 'consultation',
            title: 'Free Consultation',
            subtitle: '3D Scan + Treatment Plan',
            description: 'We examine your teeth, take digital scans, and create your personalized roadmap.',
            icon: '🦷',
            cost: 0,
            duration: { min: 0, max: 0, unit: 'days' },
            stage: 1,
            stageLabel: 'Foundation',
            always: true,
            details: [
                'Full oral examination',
                '3D digital scan',
                'Photos and X-rays if needed',
                'Treatment options explained',
                'Cost breakdown with no obligation'
            ],
            tip: 'Bring any previous dental records if you have them.'
        },
        gumHealth: {
            id: 'gumHealth',
            title: 'Gum Health Spa',
            subtitle: 'Deep Clean & Therapy',
            description: 'Professional cleaning removes plaque buildup and treats any gum issues - the foundation for lasting results.',
            icon: '✨',
            cost: { min: 150, max: 180 },
            duration: { min: 0, max: 0, unit: 'days' },
            stage: 2,
            stageLabel: 'Health First',
            goals: ['fill-gaps', 'straighten', 'brighten', 'perfect-shape', 'complete'],
            details: [
                'Ultrasonic scaling',
                'Air polish treatment',
                'Gum pocket assessment',
                'Oral hygiene coaching'
            ],
            tip: 'Most patients say this alone makes their smile look dramatically better.'
        },
        implants: {
            id: 'implants',
            title: 'Dental Implants',
            subtitle: 'Permanent Tooth Replacement',
            description: 'Titanium roots fused to your jawbone, topped with porcelain crowns indistinguishable from natural teeth.',
            icon: '🔩',
            cost: { min: 1999, max: 1999, per: 'implant' },
            duration: { min: 3, max: 6, unit: 'months' },
            stage: 3,
            stageLabel: 'Structure',
            goals: ['fill-gaps', 'complete'],
            branch: 'structure',
            link: '/dental-implants/',
            details: [
                'Implant placement (90 mins)',
                'Healing period (3-4 months)',
                'Crown fitting (2 visits)'
            ],
            tip: 'Many patients say it was less painful than expected - often easier than an extraction.'
        },
        aligners: {
            id: 'aligners',
            title: 'Clear Aligners',
            subtitle: 'Invisible Straightening',
            description: 'Custom clear trays gradually shift your teeth over 6-18 months. Removable for eating and photos.',
            icon: '😁',
            cost: { min: 2999, max: 2999 },
            duration: { min: 6, max: 18, unit: 'months' },
            stage: 3,
            stageLabel: 'Structure',
            goals: ['straighten', 'complete'],
            branch: 'structure',
            link: '/teeth-straightening/',
            details: [
                'Digital smile design',
                'Custom aligner trays',
                'Progress checks every 6-8 weeks',
                'Retainers included'
            ],
            tip: "Most people won't notice you're wearing them."
        },
        whitening: {
            id: 'whitening',
            title: 'Professional Whitening',
            subtitle: 'Hollywood-Level Brightness',
            description: 'Professional-grade whitening that goes 6-8 shades lighter than shop-bought options.',
            icon: '💎',
            cost: { min: 349, max: 549 },
            duration: { min: 0, max: 0, unit: 'days' },
            stage: 4,
            stageLabel: 'Brightness',
            goals: ['brighten', 'perfect-shape', 'complete'],
            completionPoint: true,
            details: [
                'In-office power whitening',
                'Custom take-home trays',
                'Maintenance gel supply'
            ],
            tip: 'Results last 1-3 years depending on diet and habits.'
        },
        bonding: {
            id: 'bonding',
            title: 'Composite Bonding',
            subtitle: 'Reshape & Refine',
            description: 'Tooth-colored resin sculpted directly onto your teeth to fix chips, gaps, and shape issues.',
            icon: '🎨',
            cost: { min: 299, max: 299, per: 'tooth' },
            duration: { min: 0, max: 0, unit: 'days' },
            stage: 5,
            stageLabel: 'Perfection',
            goals: ['perfect-shape'],
            branch: 'perfection',
            link: '/composite-bonding/',
            details: [
                'Color matching',
                'Tooth preparation (minimal)',
                'Resin sculpting',
                'Polish and finishing'
            ],
            tip: 'Completely reversible - no permanent tooth removal required.'
        },
        veneers: {
            id: 'veneers',
            title: 'Porcelain Veneers',
            subtitle: 'The Ultimate Transformation',
            description: 'Custom ceramic shells bonded over your teeth for the most dramatic, lasting smile transformation.',
            icon: '👑',
            cost: { min: 695, max: 695, per: 'tooth' },
            duration: { min: 0, max: 1, unit: 'months' },
            stage: 5,
            stageLabel: 'Perfection',
            goals: ['perfect-shape', 'complete'],
            branch: 'perfection',
            link: '/porcelain-veneers/',
            details: [
                'Digital smile design preview',
                'Tooth preparation',
                'Temporary veneers',
                'Custom porcelain fitting'
            ],
            tip: 'See your new smile in a digital mockup before committing.'
        }
    };

    const GOALS = {
        'fill-gaps': { label: 'Fill Gaps', icon: '🦷' },
        'straighten': { label: 'Straighten', icon: '😁' },
        'brighten': { label: 'Brighten', icon: '✨' },
        'perfect-shape': { label: 'Perfect Shape', icon: '💎' },
        'complete': { label: 'Complete Makeover', icon: '👑' },
        'exploring': { label: 'Just Exploring', icon: '🔍' }
    };

    // ═══════════════════════════════════════════════════════════
    // STATE MANAGEMENT
    // ═══════════════════════════════════════════════════════════

    const STORAGE_KEY = 'smile_journey_v1';

    const state = {
        goals: [],
        expandedNode: null,
        saved: false
    };

    function saveState() {
        localStorage.setItem(STORAGE_KEY, JSON.stringify({
            goals: state.goals,
            savedAt: Date.now()
        }));
        state.saved = true;
        updateSaveButton();
    }

    function loadState() {
        try {
            const stored = localStorage.getItem(STORAGE_KEY);
            if (!stored) return null;

            const data = JSON.parse(stored);
            // Expire after 30 days
            if (Date.now() - data.savedAt > 30 * 24 * 60 * 60 * 1000) {
                localStorage.removeItem(STORAGE_KEY);
                return null;
            }
            return data;
        } catch (e) {
            return null;
        }
    }

    function loadFromUrl() {
        const params = new URLSearchParams(window.location.search);
        const plan = params.get('plan');
        if (plan) {
            try {
                return JSON.parse(atob(plan));
            } catch (e) {
                return null;
            }
        }
        return null;
    }

    function generateShareUrl() {
        const data = btoa(JSON.stringify({ goals: state.goals }));
        return `${window.location.origin}${window.location.pathname}?plan=${data}`;
    }

    // ═══════════════════════════════════════════════════════════
    // JOURNEY BUILDER
    // ═══════════════════════════════════════════════════════════

    function getJourneyTreatments() {
        const goals = state.goals;
        if (goals.length === 0) return [];

        // Always include consultation first
        const journey = [TREATMENTS.consultation];

        // Get treatments that match selected goals
        const relevantTreatments = Object.values(TREATMENTS)
            .filter(t => !t.always && t.goals?.some(g => goals.includes(g)));

        // Sort by stage
        relevantTreatments.sort((a, b) => a.stage - b.stage);

        // Group by stage for branch handling
        const byStage = {};
        relevantTreatments.forEach(t => {
            if (!byStage[t.stage]) byStage[t.stage] = [];
            byStage[t.stage].push(t);
        });

        // Build journey with branches
        Object.keys(byStage).sort((a, b) => a - b).forEach(stage => {
            const treatments = byStage[stage];
            const branches = {};
            const nonBranch = [];

            treatments.forEach(t => {
                if (t.branch) {
                    if (!branches[t.branch]) branches[t.branch] = [];
                    branches[t.branch].push(t);
                } else {
                    nonBranch.push(t);
                }
            });

            // Add non-branching treatments
            journey.push(...nonBranch);

            // Add branch groups
            Object.values(branches).forEach(branchTreatments => {
                if (branchTreatments.length > 1) {
                    journey.push({ type: 'branch', treatments: branchTreatments });
                } else {
                    journey.push(...branchTreatments);
                }
            });
        });

        // Add destination
        journey.push({ type: 'destination' });

        return journey;
    }

    function calculateEstimate() {
        const journey = getJourneyTreatments();
        let minCost = 0;
        let maxCost = 0;
        let minMonths = 0;
        let maxMonths = 0;

        journey.forEach(item => {
            if (item.type === 'branch') {
                // For branches, use the average of options
                const avgMin = item.treatments.reduce((sum, t) => sum + (typeof t.cost === 'number' ? t.cost : t.cost?.min || 0), 0) / item.treatments.length;
                const avgMax = item.treatments.reduce((sum, t) => sum + (typeof t.cost === 'number' ? t.cost : t.cost?.max || 0), 0) / item.treatments.length;
                minCost += avgMin;
                maxCost += avgMax;

                const dMin = Math.max(...item.treatments.map(t => t.duration?.min || 0));
                const dMax = Math.max(...item.treatments.map(t => t.duration?.max || 0));
                minMonths += dMin;
                maxMonths += dMax;
            } else if (item.type !== 'destination' && item.cost !== undefined) {
                if (typeof item.cost === 'number') {
                    minCost += item.cost;
                    maxCost += item.cost;
                } else {
                    minCost += item.cost.min || 0;
                    maxCost += item.cost.max || 0;
                }

                if (item.duration) {
                    if (item.duration.unit === 'months') {
                        minMonths += item.duration.min || 0;
                        maxMonths += item.duration.max || 0;
                    }
                }
            }
        });

        return {
            minCost,
            maxCost,
            minMonths,
            maxMonths,
            monthly: Math.ceil(maxCost / 24) // 24 month finance
        };
    }

    // ═══════════════════════════════════════════════════════════
    // RENDERING
    // ═══════════════════════════════════════════════════════════

    function renderJourney() {
        const container = document.getElementById('journey-map');
        const emptyState = document.getElementById('empty-state');
        const subtitle = document.getElementById('journey-subtitle');

        const journey = getJourneyTreatments();

        if (journey.length === 0) {
            emptyState.style.display = 'block';
            container.querySelectorAll('.journey-node, .journey-branch, .journey-connector, .journey-destination').forEach(el => el.remove());
            subtitle.textContent = 'Select your goals above to build your journey';
            return;
        }

        emptyState.style.display = 'none';

        // Count selected goals for subtitle
        const goalLabels = state.goals.map(g => GOALS[g]?.label).filter(Boolean);
        if (goalLabels.length > 0) {
            subtitle.textContent = `Your path to: ${goalLabels.join(', ')}`;
        }

        // Clear existing nodes
        container.querySelectorAll('.journey-node, .journey-branch, .journey-connector, .journey-destination').forEach(el => el.remove());

        // Render journey
        journey.forEach((item, index) => {
            if (item.type === 'branch') {
                renderBranch(container, item.treatments, index);
            } else if (item.type === 'destination') {
                renderDestination(container);
            } else {
                renderNode(container, item, index);
            }

            // Add connector (except after last item)
            if (index < journey.length - 1 && item.type !== 'destination') {
                const nextItem = journey[index + 1];
                if (nextItem.type !== 'destination') {
                    const connector = document.createElement('div');
                    connector.className = 'journey-connector';
                    container.appendChild(connector);
                }
            }
        });

        // Trigger animations
        requestAnimationFrame(() => {
            container.querySelectorAll('.journey-node, .journey-branch, .journey-destination').forEach((el, i) => {
                setTimeout(() => el.classList.add('visible'), i * 100);
            });
        });

        updateEstimator();
    }

    function renderNode(container, treatment, index) {
        const node = document.createElement('div');
        node.className = 'journey-node';
        node.dataset.id = treatment.id;

        const isExpanded = state.expandedNode === treatment.id;
        if (isExpanded) node.classList.add('expanded');

        node.innerHTML = `
            <div class="node-stage">
                <span class="stage-num">${treatment.stage}</span>
                <span class="stage-label">${treatment.stageLabel}</span>
            </div>
            <div class="node-card">
                <div class="node-main" role="button" tabindex="0">
                    <div class="node-icon">${treatment.icon}</div>
                    <div class="node-info">
                        <h3>${treatment.title}</h3>
                        <p class="node-subtitle">${treatment.subtitle}</p>
                    </div>
                    <div class="node-cost">
                        ${formatCost(treatment.cost)}
                    </div>
                    <div class="node-expand-icon">${isExpanded ? '−' : '+'}</div>
                </div>
                <div class="node-details">
                    <p class="node-description">${treatment.description}</p>
                    <div class="node-details-list">
                        <h4>What happens:</h4>
                        <ul>
                            ${treatment.details?.map(d => `<li>${d}</li>`).join('') || ''}
                        </ul>
                    </div>
                    ${treatment.tip ? `<div class="node-tip"><strong>Tip:</strong> ${treatment.tip}</div>` : ''}
                    ${treatment.link ? `<a href="${treatment.link}" class="node-link">Learn more about ${treatment.title} →</a>` : ''}
                </div>
                ${treatment.completionPoint ? '<div class="completion-badge">Many patients stop here!</div>' : ''}
            </div>
        `;

        const mainArea = node.querySelector('.node-main');
        mainArea.addEventListener('click', () => toggleNodeExpansion(treatment.id));
        mainArea.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                toggleNodeExpansion(treatment.id);
            }
        });

        container.appendChild(node);
    }

    function renderBranch(container, treatments, index) {
        const branch = document.createElement('div');
        branch.className = 'journey-branch';

        const stageInfo = treatments[0];

        branch.innerHTML = `
            <div class="branch-stage">
                <span class="stage-num">${stageInfo.stage}</span>
                <span class="stage-label">${stageInfo.stageLabel}</span>
            </div>
            <div class="branch-options">
                ${treatments.map(t => `
                    <div class="branch-card" data-id="${t.id}">
                        <div class="branch-main" role="button" tabindex="0">
                            <div class="branch-icon">${t.icon}</div>
                            <h3>${t.title}</h3>
                            <p class="branch-subtitle">${t.subtitle}</p>
                            <div class="branch-cost">${formatCost(t.cost)}</div>
                        </div>
                        <div class="branch-details">
                            <p>${t.description}</p>
                            ${t.link ? `<a href="${t.link}" class="node-link">Learn more →</a>` : ''}
                        </div>
                    </div>
                `).join('<div class="branch-or">or</div>')}
            </div>
        `;

        branch.querySelectorAll('.branch-main').forEach(el => {
            el.addEventListener('click', () => {
                const card = el.closest('.branch-card');
                card.classList.toggle('expanded');
            });
        });

        container.appendChild(branch);
    }

    function renderDestination(container) {
        const dest = document.createElement('div');
        dest.className = 'journey-destination';
        dest.innerHTML = `
            <div class="destination-sparkles">
                <span class="sparkle s1">✨</span>
                <span class="sparkle s2">✨</span>
                <span class="sparkle s3">✨</span>
            </div>
            <div class="destination-icon">🌟</div>
            <h3>Your Dream Smile</h3>
            <p>Confidence that lasts a lifetime</p>
        `;
        container.appendChild(dest);
    }

    function formatCost(cost) {
        if (cost === 0 || cost === undefined) return '<span class="free">Free</span>';
        if (typeof cost === 'number') return `£${cost.toLocaleString()}`;

        const per = cost.per ? `<span class="per">/${cost.per}</span>` : '';
        if (cost.min === cost.max) {
            return `£${cost.min.toLocaleString()}${per}`;
        }
        return `£${cost.min.toLocaleString()} - £${cost.max.toLocaleString()}${per}`;
    }

    function updateEstimator() {
        const est = calculateEstimate();
        const estimator = document.getElementById('estimator');
        const emptyEl = document.getElementById('estimator-empty');
        const bodyEl = document.getElementById('estimator-body');
        const actionsEl = document.getElementById('estimator-actions');
        const noteEl = document.getElementById('estimator-note');

        if (state.goals.length === 0) {
            estimator.classList.remove('active');
            emptyEl.style.display = 'block';
            bodyEl.style.display = 'none';
            actionsEl.style.display = 'none';
            noteEl.style.display = 'none';
            return;
        }

        estimator.classList.add('active');
        emptyEl.style.display = 'none';
        bodyEl.style.display = 'block';
        actionsEl.style.display = 'flex';
        noteEl.style.display = 'block';

        // Timeline
        let timeline = '';
        if (est.maxMonths === 0) {
            timeline = 'Same day - 2 weeks';
        } else if (est.minMonths === est.maxMonths) {
            timeline = `~${est.maxMonths} months`;
        } else {
            timeline = `${est.minMonths}-${est.maxMonths} months`;
        }
        document.getElementById('est-timeline').textContent = timeline;

        // Cost
        let costText = '';
        if (est.minCost === est.maxCost) {
            costText = `£${est.maxCost.toLocaleString()}+`;
        } else {
            costText = `£${est.minCost.toLocaleString()} - £${est.maxCost.toLocaleString()}`;
        }
        document.getElementById('est-cost').textContent = costText;

        // Monthly
        document.getElementById('est-monthly').textContent = est.monthly > 0 ? `~£${est.monthly}/mo` : '—';

        // Pulse animation
        estimator.classList.add('updated');
        setTimeout(() => estimator.classList.remove('updated'), 400);
    }

    function updateSaveButton() {
        const btn = document.getElementById('btn-save');
        if (state.saved) {
            btn.innerHTML = '<span class="save-icon">✓</span> Journey Saved';
            btn.classList.add('saved');
        } else {
            btn.innerHTML = '<span class="save-icon">💾</span> Save My Journey';
            btn.classList.remove('saved');
        }
    }

    function toggleNodeExpansion(id) {
        if (state.expandedNode === id) {
            state.expandedNode = null;
        } else {
            state.expandedNode = id;
        }

        document.querySelectorAll('.journey-node').forEach(node => {
            const isTarget = node.dataset.id === id;
            const shouldExpand = isTarget && state.expandedNode === id;

            node.classList.toggle('expanded', shouldExpand);
            node.querySelector('.node-expand-icon').textContent = shouldExpand ? '−' : '+';

            if (shouldExpand) {
                node.classList.remove('dimmed');
            } else if (state.expandedNode) {
                node.classList.add('dimmed');
            } else {
                node.classList.remove('dimmed');
            }
        });
    }

    // ═══════════════════════════════════════════════════════════
    // EVENT HANDLERS
    // ═══════════════════════════════════════════════════════════

    function toggleGoal(goal) {
        const index = state.goals.indexOf(goal);
        if (index > -1) {
            state.goals.splice(index, 1);
        } else {
            // 'exploring' is exclusive
            if (goal === 'exploring') {
                state.goals = ['exploring'];
            } else {
                state.goals = state.goals.filter(g => g !== 'exploring');
                state.goals.push(goal);
            }
        }

        state.saved = false;
        updateGoalCards();
        renderJourney();
        updateSaveButton();

        // Scroll to journey on first selection and show hint
        if (state.goals.length === 1) {
            setTimeout(() => {
                document.getElementById('journey-section').scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
                // Show hint after scroll completes
                setTimeout(() => {
                    showHint('You can scroll back up to add more goals anytime');
                }, 800);
            }, 300);
        }
    }

    function updateGoalCards() {
        document.querySelectorAll('.goal-card').forEach(card => {
            const goal = card.dataset.goal;
            const isSelected = state.goals.includes(goal);
            card.classList.toggle('selected', isSelected);
            card.setAttribute('aria-pressed', isSelected);
        });
    }

    function copyShareUrl() {
        const url = generateShareUrl();
        navigator.clipboard.writeText(url).then(() => {
            const btn = document.getElementById('btn-share');
            const original = btn.innerHTML;
            btn.innerHTML = '<span>✓</span> Link Copied!';
            setTimeout(() => btn.innerHTML = original, 2000);
        });
    }

    // ═══════════════════════════════════════════════════════════
    // INITIALIZATION
    // ═══════════════════════════════════════════════════════════

    document.addEventListener('DOMContentLoaded', () => {
        // Check for shared URL first
        const urlState = loadFromUrl();
        if (urlState && urlState.goals) {
            state.goals = urlState.goals;
        } else {
            // Then check localStorage
            const stored = loadState();
            if (stored && stored.goals) {
                state.goals = stored.goals;
                state.saved = true;
            }
        }

        // Set up goal card listeners
        document.querySelectorAll('.goal-card').forEach(card => {
            card.addEventListener('click', () => toggleGoal(card.dataset.goal));
        });

        // Save button
        document.getElementById('btn-save').addEventListener('click', saveState);

        // Share button
        document.getElementById('btn-share').addEventListener('click', copyShareUrl);

        // Initial render
        updateGoalCards();
        renderJourney();
        updateSaveButton();

        // Show welcome back message if returning
        if (state.saved && state.goals.length > 0) {
            showToast('Welcome back! Your journey is ready.');
        }

        // Photo upload handling (keep existing)
        const selfieInput = document.getElementById('selfie');
        if (selfieInput) {
            selfieInput.addEventListener('change', function(e) {
                const file = e.target.files[0];
                if (file) {
                    const reader = new FileReader();
                    reader.onload = function(event) {
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
                        setTimeout(() => {
                            btn.scrollIntoView({ behavior: 'smooth', block: 'center' });
                            setTimeout(() => btn.classList.add('glow-pulse'), 450);
                        }, 500);
                    };
                    reader.readAsDataURL(file);
                }
            });
        }

        // HTMX after swap
        document.body.addEventListener('htmx:afterSwap', function(e) {
            if (e.detail.target.id === 'tool-content') {
                setTimeout(() => {
                    const result = document.querySelector('.smile-result');
                    if (result) {
                        result.scrollIntoView({ behavior: 'smooth', block: 'center' });
                    }
                }, 100);
            }
        });

        // Mobile drawer tap-to-toggle
        if (window.innerWidth <= 900) {
            const estimator = document.getElementById('estimator');
            const header = estimator.querySelector('.estimator-header');

            header.addEventListener('click', (e) => {
                e.stopPropagation();
                estimator.classList.toggle('drawer-open');
            });

            // Close when clicking outside
            document.addEventListener('click', (e) => {
                if (!estimator.contains(e.target)) {
                    estimator.classList.remove('drawer-open');
                }
            });
        }
    });

    function showToast(message) {
        const toast = document.createElement('div');
        toast.className = 'toast';
        toast.textContent = message;
        document.querySelector('main.smile-journey').appendChild(toast);

        requestAnimationFrame(() => toast.classList.add('visible'));

        setTimeout(() => {
            toast.classList.remove('visible');
            setTimeout(() => toast.remove(), 300);
        }, 3000);
    }

    function showHint(message) {
        const hint = document.createElement('div');
        hint.className = 'top-hint';
        hint.innerHTML = `<span>💡</span> ${message}`;
        document.querySelector('main.smile-journey').appendChild(hint);

        requestAnimationFrame(() => hint.classList.add('visible'));

        setTimeout(() => {
            hint.classList.remove('visible');
            setTimeout(() => hint.remove(), 300);
        }, 4000);
    }
    "##
}

fn css(site: &mut Site<UCDPages>) {
    site.declare_css("smile-journey", r##"

        @keyframes float {
            0%, 100% { transform: translateY(0); }
            50% { transform: translateY(-8px); }
        }

        @keyframes sparkle-float {
            0% { opacity: 0; transform: translateY(0) scale(0); }
            50% { opacity: 1; transform: translateY(-20px) scale(1); }
            100% { opacity: 0; transform: translateY(-40px) scale(0); }
        }

        @keyframes checkmark-pop {
            0% { transform: scale(0); }
            50% { transform: scale(1.3); }
            100% { transform: scale(1); }
        }

        @keyframes pulse-border {
            0%, 100% { border-color: var(--turquoise-70); }
            50% { border-color: var(--turquoise-30); }
        }

        @keyframes glow-pulse {
            0% { box-shadow: 0 0 0 0 rgba(104, 248, 253, 0.8); }
            70% { box-shadow: 0 0 0 15px rgba(104, 248, 253, 0); }
            100% { box-shadow: 0 0 0 0 rgba(104, 248, 253, 0); }
        }

        @keyframes spin {
            to { transform: rotate(360deg); }
        }

        @keyframes slide-up {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }

        @keyframes toast-in {
            from { opacity: 0; transform: translateX(-50%) translateY(20px); }
            to { opacity: 1; transform: translateX(-50%) translateY(0); }
        }

        main.smile-journey {
            background: linear-gradient(180deg, #f8f9fa 0%, #fff 100%);

            /* ═══════════════════════════════════════════════════════════ */
            /* HERO SECTION */
            /* ═══════════════════════════════════════════════════════════ */

            .hero-section {
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
                    max-width: 1100px;
                    margin: 0 auto;
                    position: relative;
                    z-index: 10;
                    padding: 160px var(--site-padding-x) 60px;
                }

                .text-area {
                    text-align: center;
                    margin-bottom: 48px;

                    .eyebrow {
                        font-size: 14px;
                        text-transform: uppercase;
                        letter-spacing: 2px;
                        color: var(--turquoise-30);
                        margin-bottom: 16px;
                        font-weight: 600;
                    }

                    h1 {
                        font-size: 48px;
                        margin-bottom: 16px;
                        font-weight: 700;
                        line-height: 1.1;
                        color: var(--turquoise-15);

                        span {
                            color: var(--turquoise-30);
                            font-style: italic;
                        }

                        @media (max-width: 768px) {
                            font-size: 32px;
                        }
                    }

                    .subtitle {
                        font-size: 18px;
                        line-height: 1.6;
                        color: var(--grey-50);
                        margin: 0;
                    }
                }

                .goal-cards {
                    display: grid;
                    grid-template-columns: repeat(3, 1fr);
                    gap: 20px;

                    @media (max-width: 900px) {
                        grid-template-columns: repeat(2, 1fr);
                    }

                    @media (max-width: 500px) {
                        grid-template-columns: 1fr;
                    }
                }

                .goal-card {
                    position: relative;
                    padding: 28px 24px;
                    background: rgba(255, 255, 255, 0.9);
                    backdrop-filter: blur(10px);
                    border: 2px solid var(--turquoise-90);
                    border-radius: 16px;
                    cursor: pointer;
                    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                    text-align: center;

                    &::before {
                        content: '';
                        position: absolute;
                        inset: -3px;
                        border-radius: 18px;
                        background: linear-gradient(135deg, var(--turquoise-70), var(--turquoise-30));
                        opacity: 0;
                        z-index: -1;
                        filter: blur(12px);
                        transition: opacity 0.3s ease;
                    }

                    &:hover {
                        transform: translateY(-6px);
                        border-color: var(--turquoise-70);
                        box-shadow: 0 12px 32px rgba(0, 111, 115, 0.12);

                        &::before {
                            opacity: 0.4;
                        }
                    }

                    &.selected {
                        border-color: var(--turquoise-30);
                        background: var(--turquoise-98);

                        &::after {
                            content: '✓';
                            position: absolute;
                            top: 12px;
                            right: 12px;
                            width: 26px;
                            height: 26px;
                            background: var(--turquoise-30);
                            color: white;
                            border-radius: 50%;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            font-size: 14px;
                            font-weight: 600;
                            animation: checkmark-pop 0.3s ease;
                        }
                    }

                    .goal-icon {
                        font-size: 40px;
                        margin-bottom: 12px;
                    }

                    h3 {
                        font-size: 18px;
                        color: var(--turquoise-15);
                        margin-bottom: 4px;
                        font-weight: 600;
                    }

                    p {
                        font-size: 14px;
                        color: var(--grey-50);
                        margin: 0;
                    }
                }
            }

            /* ═══════════════════════════════════════════════════════════ */
            /* JOURNEY SECTION */
            /* ═══════════════════════════════════════════════════════════ */

            .journey-section {
                padding: 80px 0;

                .inner {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 0 var(--site-padding-x);
                }

                .journey-header {
                    text-align: center;
                    margin-bottom: 48px;

                    h2 {
                        font-size: 36px;
                        color: var(--turquoise-15);
                        margin-bottom: 12px;

                        @media (max-width: 768px) {
                            font-size: 28px;
                        }
                    }

                    .journey-subtitle {
                        font-size: 18px;
                        color: var(--grey-50);
                        margin: 0;
                    }
                }

                .journey-layout {
                    display: grid;
                    grid-template-columns: 1fr 320px;
                    gap: 48px;

                    @media (max-width: 900px) {
                        grid-template-columns: 1fr;
                    }
                }

                .journey-map {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .empty-state {
                    text-align: center;
                    padding: 80px 40px;
                    background: var(--turquoise-98);
                    border-radius: 20px;
                    border: 2px dashed var(--turquoise-70);
                    width: 100%;

                    .empty-icon {
                        font-size: 64px;
                        margin-bottom: 16px;
                    }

                    p {
                        font-size: 18px;
                        color: var(--turquoise-15);
                        margin: 0 0 8px;
                    }

                    .hint {
                        font-size: 14px;
                        color: var(--grey-50);
                    }
                }

                /* Journey Node */
                .journey-node {
                    width: 100%;
                    max-width: 560px;
                    opacity: 0;
                    transform: translateY(30px);
                    transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);

                    &.visible {
                        opacity: 1;
                        transform: translateY(0);
                    }

                    &.dimmed {
                        opacity: 0.4;
                    }

                    .node-stage {
                        display: flex;
                        align-items: center;
                        gap: 10px;
                        margin-bottom: 12px;

                        .stage-num {
                            width: 28px;
                            height: 28px;
                            background: var(--turquoise-30);
                            color: white;
                            border-radius: 50%;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            font-weight: 700;
                            font-size: 14px;
                        }

                        .stage-label {
                            font-size: 12px;
                            text-transform: uppercase;
                            letter-spacing: 1.5px;
                            color: var(--grey-50);
                            font-weight: 600;
                        }
                    }

                    .node-card {
                        background: white;
                        border-radius: 16px;
                        box-shadow: 0 4px 20px rgba(0, 111, 115, 0.08);
                        border: 2px solid transparent;
                        transition: all 0.3s ease;
                        overflow: hidden;

                        &:hover {
                            border-color: var(--turquoise-70);
                            box-shadow: 0 8px 32px rgba(0, 111, 115, 0.12);
                        }
                    }

                    .node-main {
                        display: grid;
                        grid-template-columns: auto 1fr auto auto;
                        align-items: center;
                        gap: 16px;
                        padding: 20px 24px;
                        cursor: pointer;

                        @media (max-width: 500px) {
                            grid-template-columns: auto 1fr auto;

                            .node-cost {
                                grid-column: 2;
                                grid-row: 2;
                            }

                            .node-expand-icon {
                                grid-row: span 2;
                            }
                        }
                    }

                    .node-icon {
                        font-size: 36px;
                    }

                    .node-info {
                        h3 {
                            font-size: 18px;
                            color: var(--turquoise-15);
                            margin-bottom: 2px;
                        }

                        .node-subtitle {
                            font-size: 14px;
                            color: var(--grey-50);
                            margin: 0;
                        }
                    }

                    .node-cost {
                        font-size: 16px;
                        font-weight: 600;
                        color: var(--turquoise-15);

                        .free {
                            color: var(--turquoise-30);
                        }

                        .per {
                            font-size: 12px;
                            font-weight: 400;
                            color: var(--grey-50);
                        }
                    }

                    .node-expand-icon {
                        width: 32px;
                        height: 32px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        background: var(--turquoise-98);
                        border-radius: 8px;
                        color: var(--turquoise-30);
                        font-size: 20px;
                        font-weight: 600;
                        transition: all 0.2s ease;
                    }

                    .node-details {
                        max-height: 0;
                        overflow: hidden;
                        transition: max-height 0.4s cubic-bezier(0.4, 0, 0.2, 1);
                        background: var(--turquoise-98);

                        .node-description {
                            padding: 20px 24px 0;
                            font-size: 15px;
                            line-height: 1.6;
                            color: var(--grey-50);
                            margin: 0;
                        }

                        .node-details-list {
                            padding: 16px 24px;

                            h4 {
                                font-size: 13px;
                                text-transform: uppercase;
                                letter-spacing: 1px;
                                color: var(--turquoise-30);
                                margin-bottom: 8px;
                            }

                            ul {
                                margin: 0;
                                padding-left: 20px;

                                li {
                                    font-size: 14px;
                                    color: var(--grey-50);
                                    margin-bottom: 4px;
                                }
                            }
                        }

                        .node-tip {
                            margin: 0 24px 16px;
                            padding: 12px 16px;
                            background: white;
                            border-radius: 8px;
                            font-size: 14px;
                            color: var(--grey-50);

                            strong {
                                color: var(--turquoise-30);
                            }
                        }

                        .node-link {
                            display: block;
                            padding: 16px 24px;
                            background: white;
                            color: var(--turquoise-30);
                            font-weight: 600;
                            font-size: 14px;
                            text-decoration: none;
                            transition: background 0.2s ease;

                            &:hover {
                                background: var(--turquoise-90);
                            }
                        }
                    }

                    &.expanded {
                        .node-card {
                            border-color: var(--turquoise-30);
                        }

                        .node-expand-icon {
                            background: var(--turquoise-30);
                            color: white;
                        }

                        .node-details {
                            max-height: 500px;
                        }
                    }

                    .completion-badge {
                        background: var(--turquoise-90);
                        color: var(--turquoise-15);
                        text-align: center;
                        padding: 10px;
                        font-size: 13px;
                        font-weight: 600;
                    }
                }

                /* Journey Connector */
                .journey-connector {
                    width: 4px;
                    height: 48px;
                    background: linear-gradient(180deg, var(--turquoise-30), var(--turquoise-70));
                    border-radius: 2px;
                    margin: 8px 0;
                }

                /* Journey Branch */
                .journey-branch {
                    width: 100%;
                    max-width: 700px;
                    opacity: 0;
                    transform: translateY(30px);
                    transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);

                    &.visible {
                        opacity: 1;
                        transform: translateY(0);
                    }

                    .branch-stage {
                        display: flex;
                        align-items: center;
                        gap: 10px;
                        margin-bottom: 12px;
                        justify-content: center;

                        .stage-num {
                            width: 28px;
                            height: 28px;
                            background: var(--turquoise-30);
                            color: white;
                            border-radius: 50%;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            font-weight: 700;
                            font-size: 14px;
                        }

                        .stage-label {
                            font-size: 12px;
                            text-transform: uppercase;
                            letter-spacing: 1.5px;
                            color: var(--grey-50);
                            font-weight: 600;
                        }
                    }

                    .branch-options {
                        display: flex;
                        align-items: stretch;
                        justify-content: center;
                        gap: 16px;

                        @media (max-width: 600px) {
                            flex-direction: column;
                            align-items: center;
                        }
                    }

                    .branch-or {
                        display: flex;
                        align-items: center;
                        font-size: 14px;
                        color: var(--grey-50);
                        font-weight: 600;
                        text-transform: uppercase;
                        letter-spacing: 1px;
                    }

                    .branch-card {
                        flex: 1;
                        background: white;
                        border-radius: 16px;
                        box-shadow: 0 4px 20px rgba(0, 111, 115, 0.08);
                        border: 2px solid transparent;
                        transition: all 0.3s ease;
                        overflow: hidden;
                        max-width: 300px;

                        @media (max-width: 600px) {
                            max-width: 100%;
                            width: 100%;
                        }

                        &:hover {
                            border-color: var(--turquoise-70);
                            transform: translateY(-4px);
                        }

                        &.expanded {
                            border-color: var(--turquoise-30);

                            .branch-details {
                                max-height: 200px;
                                padding: 16px;
                            }
                        }
                    }

                    .branch-main {
                        padding: 24px;
                        text-align: center;
                        cursor: pointer;
                    }

                    .branch-icon {
                        font-size: 40px;
                        margin-bottom: 12px;
                    }

                    .branch-card h3 {
                        font-size: 18px;
                        color: var(--turquoise-15);
                        margin-bottom: 4px;
                    }

                    .branch-subtitle {
                        font-size: 13px;
                        color: var(--grey-50);
                        margin: 0 0 12px;
                    }

                    .branch-cost {
                        font-size: 16px;
                        font-weight: 600;
                        color: var(--turquoise-15);

                        .per {
                            font-size: 12px;
                            font-weight: 400;
                            color: var(--grey-50);
                        }
                    }

                    .branch-details {
                        max-height: 0;
                        overflow: hidden;
                        transition: all 0.3s ease;
                        padding: 0 16px;
                        background: var(--turquoise-98);

                        p {
                            font-size: 14px;
                            color: var(--grey-50);
                            line-height: 1.5;
                            margin: 0 0 12px;
                        }

                        .node-link {
                            display: inline-block;
                            color: var(--turquoise-30);
                            font-weight: 600;
                            font-size: 14px;
                            text-decoration: none;

                            &:hover {
                                text-decoration: underline;
                            }
                        }
                    }
                }

                /* Journey Destination */
                .journey-destination {
                    position: relative;
                    background: linear-gradient(135deg, var(--turquoise-15), var(--turquoise-30));
                    color: white;
                    padding: 48px 64px;
                    border-radius: 20px;
                    text-align: center;
                    opacity: 0;
                    transform: translateY(30px);
                    transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
                    margin-top: 32px;

                    &.visible {
                        opacity: 1;
                        transform: translateY(0);
                    }

                    .destination-icon {
                        font-size: 56px;
                        margin-bottom: 12px;
                        animation: float 3s ease-in-out infinite;
                    }

                    h3 {
                        font-size: 28px;
                        color: white;
                        margin-bottom: 8px;
                    }

                    p {
                        font-size: 16px;
                        color: var(--turquoise-90);
                        margin: 0;
                    }

                    .destination-sparkles {
                        position: absolute;
                        inset: 0;
                        pointer-events: none;

                        .sparkle {
                            position: absolute;
                            font-size: 24px;
                            animation: sparkle-float 2s ease-in-out infinite;

                            &.s1 { top: 15%; left: 15%; animation-delay: 0s; }
                            &.s2 { top: 25%; right: 18%; animation-delay: 0.6s; }
                            &.s3 { bottom: 25%; left: 22%; animation-delay: 1.2s; }
                        }
                    }

                    @media (max-width: 500px) {
                        padding: 36px 28px;

                        h3 {
                            font-size: 24px;
                        }
                    }
                }
            }

            /* ═══════════════════════════════════════════════════════════ */
            /* ESTIMATOR */
            /* ═══════════════════════════════════════════════════════════ */

            .estimator {
                position: sticky;
                top: 100px;
                align-self: start;
                background: white;
                border-radius: 20px;
                box-shadow: 0 8px 32px rgba(0, 111, 115, 0.1);
                border: 2px solid var(--turquoise-90);
                overflow: hidden;
                opacity: 0.6;
                transition: all 0.3s ease;

                &.active {
                    opacity: 1;
                    border-color: var(--turquoise-70);
                }

                &.updated {
                    animation: pulse-border 0.4s ease;
                }

                .estimator-empty {
                    padding: 32px 24px;
                    text-align: center;

                    p {
                        font-size: 15px;
                        color: var(--grey-50);
                        margin: 0;
                        line-height: 1.5;
                    }
                }

                .estimator-header {
                    background: var(--turquoise-15);
                    padding: 20px 24px;

                    h3 {
                        color: white;
                        font-size: 16px;
                        margin: 0;
                        font-weight: 600;
                    }
                }

                .estimator-body {
                    padding: 24px;
                }

                .estimate-row {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 12px 0;
                    border-bottom: 1px solid var(--turquoise-90);

                    &:last-child {
                        border-bottom: none;
                    }

                    &.highlight {
                        background: var(--turquoise-98);
                        margin: 12px -24px -24px;
                        padding: 16px 24px;
                        border-bottom: none;
                    }

                    .label {
                        font-size: 14px;
                        color: var(--grey-50);
                    }

                    .value {
                        font-size: 18px;
                        font-weight: 700;
                        color: var(--turquoise-15);
                        transition: color 0.2s ease;
                    }
                }

                .estimator-actions {
                    padding: 0 24px 24px;
                    display: flex;
                    flex-direction: column;
                    gap: 12px;
                }

                .estimator-note {
                    padding: 16px 24px;
                    background: var(--turquoise-98);

                    p {
                        font-size: 12px;
                        color: var(--grey-50);
                        margin: 0;
                        text-align: center;
                    }
                }

                @media (max-width: 900px) {
                    position: fixed;
                    top: auto;
                    bottom: 0;
                    left: 0;
                    right: 0;
                    border-radius: 20px 20px 0 0;
                    z-index: 100;
                    max-height: 80vh;
                    overflow-y: auto;
                    transform: translateY(calc(100% - 140px));
                    transition: transform 0.3s ease;

                    &.drawer-open,
                    &:focus-within {
                        transform: translateY(0);
                    }

                    .estimator-header {
                        position: sticky;
                        top: 0;
                        z-index: 1;
                        cursor: pointer;

                        &::before {
                            content: '';
                            display: block;
                            width: 40px;
                            height: 4px;
                            background: rgba(255,255,255,0.4);
                            border-radius: 2px;
                            margin: 0 auto 12px;
                        }
                    }
                }
            }

            /* ═══════════════════════════════════════════════════════════ */
            /* BUTTONS */
            /* ═══════════════════════════════════════════════════════════ */

            .btn-primary {
                display: inline-flex;
                align-items: center;
                justify-content: center;
                gap: 8px;
                padding: 16px 28px;
                background: var(--turquoise-15);
                color: white;
                border-radius: 10px;
                font-weight: 600;
                font-size: 15px;
                text-decoration: none;
                border: none;
                cursor: pointer;
                transition: all 0.3s ease;

                &:hover {
                    background: var(--turquoise-30);
                    transform: translateY(-2px);
                    color: white;
                }

                &.glow-pulse {
                    animation: glow-pulse 0.8s ease-out;
                }
            }

            .btn-secondary {
                display: inline-flex;
                align-items: center;
                justify-content: center;
                gap: 8px;
                padding: 14px 24px;
                background: white;
                color: var(--turquoise-30);
                border: 2px solid var(--turquoise-70);
                border-radius: 10px;
                font-weight: 600;
                font-size: 14px;
                cursor: pointer;
                transition: all 0.3s ease;

                &:hover {
                    background: var(--turquoise-98);
                    border-color: var(--turquoise-30);
                }

                &.saved {
                    background: var(--turquoise-90);
                    border-color: var(--turquoise-30);
                    color: var(--turquoise-15);
                }
            }

            /* ═══════════════════════════════════════════════════════════ */
            /* PREVIEW SECTION */
            /* ═══════════════════════════════════════════════════════════ */

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
                        font-size: 36px;
                        margin-bottom: 12px;

                        @media (max-width: 768px) {
                            font-size: 28px;
                        }
                    }

                    p {
                        color: var(--turquoise-70);
                        font-size: 18px;
                        margin: 0;
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
                            font-size: 18px;
                            font-weight: 600;
                            color: var(--turquoise-15);
                        }

                        .upload-hint {
                            font-size: 14px;
                            color: var(--grey-50);
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
                        font-size: 16px;
                    }
                }
            }

            /* ═══════════════════════════════════════════════════════════ */
            /* CTA SECTION */
            /* ═══════════════════════════════════════════════════════════ */

            .cta-section {
                padding: 80px 0;

                .inner {
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 0 var(--site-padding-x);
                }

                .cta-card {
                    background: var(--turquoise-98);
                    border-radius: 24px;
                    padding: 56px;
                    text-align: center;
                    border: 2px solid var(--turquoise-70);

                    @media (max-width: 768px) {
                        padding: 36px 24px;
                    }

                    h2 {
                        font-size: 32px;
                        color: var(--turquoise-15);
                        margin-bottom: 16px;

                        @media (max-width: 768px) {
                            font-size: 24px;
                        }
                    }

                    p {
                        font-size: 18px;
                        color: var(--grey-50);
                        margin-bottom: 28px;
                    }

                    .cta-buttons {
                        display: flex;
                        gap: 16px;
                        justify-content: center;
                        flex-wrap: wrap;
                    }
                }
            }

            /* ═══════════════════════════════════════════════════════════ */
            /* TOAST */
            /* ═══════════════════════════════════════════════════════════ */

            .toast {
                position: fixed;
                bottom: 24px;
                left: 50%;
                transform: translateX(-50%) translateY(20px);
                background: var(--turquoise-15);
                color: white;
                padding: 16px 28px;
                border-radius: 12px;
                font-size: 15px;
                font-weight: 500;
                z-index: 1000;
                opacity: 0;
                transition: all 0.3s ease;
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);

                &.visible {
                    opacity: 1;
                    transform: translateX(-50%) translateY(0);
                }

                @media (max-width: 900px) {
                    bottom: 160px;
                }
            }

            /* ═══════════════════════════════════════════════════════════ */
            /* TOP HINT */
            /* ═══════════════════════════════════════════════════════════ */

            .top-hint {
                position: fixed;
                top: 80px;
                left: 50%;
                transform: translateX(-50%) translateY(-20px);
                background: white;
                color: var(--turquoise-15);
                padding: 12px 24px;
                border-radius: 12px;
                font-size: 14px;
                font-weight: 500;
                z-index: 1000;
                opacity: 0;
                transition: all 0.3s ease;
                box-shadow: 0 4px 20px rgba(0, 111, 115, 0.15);
                border: 1px solid var(--turquoise-70);

                span {
                    margin-right: 6px;
                }

                &.visible {
                    opacity: 1;
                    transform: translateX(-50%) translateY(0);
                }
            }
        }
    "##);
}
