use crate::prelude::*;

pub fn add_head_code<T, I>(site: &mut Site<T, I>) {
    site.declare_placement(PlacementPosition::Analytics, r#"<script>(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
})(window,document,'script','dataLayer','GTM-M6FTQVZK');</script>"#);

    site.declare_placement(PlacementPosition::BodyTop, r#"<noscript><iframe src="https://www.googletagmanager.com/ns.html?id=GTM-M6FTQVZK"
height="0" width="0" style="display:none;visibility:hidden"></iframe></noscript>"#);

    site.declare_placement(PlacementPosition::Scripts, r#"<script>
    document.addEventListener('DOMContentLoaded', function() {
        const header = document.querySelector('header');
        const hero = document.querySelector('.dentist-splash-hero');
        
        if (!header || !hero) return;
        
        // Start transparent
        header.classList.add('transparent');
        
        const observer = new IntersectionObserver(
            (entries) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        header.classList.add('transparent');
                    } else {
                        header.classList.remove('transparent');
                    }
                });
            },
            { threshold: 0, rootMargin: '-80px 0px 0px 0px' }
        );
        
        observer.observe(hero);
    });
    </script>"#);
    
    site.declare_placement(PlacementPosition::BodyBottom, 
    r#"<script defer src="https://cdn.jsdelivr.net/npm/quicklink@3.0.1/dist/quicklink.umd.js"></script>
    <script>
    window.addEventListener('load', () => {
        quicklink.listen();
    });
    </script>"#
    );
}