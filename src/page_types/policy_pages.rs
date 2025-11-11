use crate::prelude::*;

pub fn construct_policy_pages(site: &mut Site<UCDPages>, page: &mut Page<UCDPages>) {


    let head = site.construct_head(page);
    let header = construct_header(site, &page.foundation);
    let footer = construct_footer(site);
    let content = policy_page_content(page);
    add_default_og_image(page);
    
    
    let html = format!(r##"
    
    <!DOCTYPE html>
        <html lang="en-GB">
        {head}
        <body>
            {header}
            <main class="policy-page">
            <section class="policy">
                <div class="polka-dots"></div>
                <div class="background-fade"></div>
                <div class="inner">
                    <div class="text-area">
                        {content}
                    </div>
                </div>
            </main>
            {footer}
        </body>
        </html>

    "##);
    
    css(site);
    
    page.foundation.content = Some(html);
    
    
}


fn css(site: &mut Site<UCDPages>) {

    site.declare_css("policy-pages", r##"
    
    {}
        
        main.policy-page {
            
            
            
            section.policy {
                
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
                    position: relative;
                    z-index: 2;
                    margin: 0 auto;
                    padding: 200px var(--site-padding-x) 96px;
                    max-width: 702px;
                }
                
                .text-area {
                
                    h1 {
                    
                        font-size: 38px;
                        margin-bottom: 32px;
                    
                    }
                    
                    
                    ul {
                    
                        padding-left: 2em;
                        margin-bottom: 28px;
                        
                        ol, li {
                        
                            p {
                            
                                margin-bottom: 0.3em;
                            
                            }
                        
                        
                        }
                    
                    }
                
                }
            }
        }    
        
    "##);


}


fn policy_page_content(page: &Page<UCDPages>) -> String {
    match page.foundation.slug.as_deref() {
        Some("privacy-policy") => {
            r#"
            
            <h1>Privacy Policy</h1>
<p>This privacy policy ("Policy") describes how We or Us collects, protects and uses the personally identifiable information ("Personal Information") you ("User", "you" or "your") may provide on the website and any of its products or services (collectively, "Website" or "Services"). It also describes the choices available to you regarding our use of your Personal Information and how you can access and update this information. This Policy does not apply to the practices of companies that we do not own or control, or to individuals that we do not employ or manage.</p>
<h3>Business Model Disclosure</h3>
<p>UrgentCare Dental operates as a marketing company that connects patients requiring emergency dental care with qualified dental practitioners in their area. We do not provide direct dental services but facilitate patient referrals to licensed dental professionals and practices.</p>
<h3>Collection of personal information</h3>
<p>We receive and store any information you knowingly provide to us when you fill any online forms on the Website. You can choose not to provide us with certain information, but then you may not be able to take advantage of some of the Website's features. Users who are uncertain about what information is mandatory are welcome to contact us.</p>
<h3>Collection of non-personal information</h3>
<p>When you visit the Website our servers automatically record information that your browser sends. This data may include information such as your device's IP address, browser type and version, operating system type and version, language preferences or the webpage you were visiting before you came to our Website, pages of our Website that you visit, the time spent on those pages, the information you search for on our Website, access times and dates, and other statistics.</p>
<h3>Use and processing of collected information</h3>
<p>Any of the information we collect from you may be used to personalize your experience; improve our Website; send notification emails such as password reminders, updates, etc; run and operate our Website and Services. Non-Personal Information collected is used only to identify potential cases of abuse and establish statistical information regarding Website usage. This statistical information is not otherwise aggregated in such a way that would identify any particular user of the system. We may process Personal Information related to you if one of the following applies: (i) You have given their consent for one or more specific purposes. Note that under some legislations we may be allowed to process information until you object to such processing (by opting out), without having to rely on consent or any other of the following legal bases below. This, however, does not apply, whenever the processing of Personal Information is subject to European data protection law; (ii) Provision of information is necessary for the performance of an agreement with you and/or for any pre-contractual obligations thereof; (ii) Processing is necessary for compliance with a legal obligation to which you are subject; (iv) Processing is related to a task that is carried out in the public interest or in the exercise of official authority vested in us; (v) Processing is necessary for the purposes of the legitimate interests pursued by us or by a third party. In any case, we will be happy to clarify the specific legal basis that applies to the processing, and in particular whether the provision of Personal Data is a statutory or contractual requirement, or a requirement necessary to enter into a contract.</p>
<h3>Information transfer and storage</h3>
<p>Depending on your location, data transfers may involve transferring and storing your information in a country other than your own. You are entitled to learn about the legal basis of information transfers to a country outside the European Union or to any international organization governed by public international law or set up by two or more countries, such as the UN, and about the security measures taken by us to safeguard your information. If any such transfer takes place, you can find out more by checking the relevant sections of this document or inquire with us using the information provided in the contact section.</p>
<h3>The rights of users</h3>
<p>You may exercise certain rights regarding your information processed by us. In particular, you have the right to do the following: (i) you have the right to withdraw consent where you have previously given your consent to the processing of your information; (ii) you have the right to object to the processing of your information if the processing is carried out on a legal basis other than consent; (iii) you have the right to learn if information is being processed by us, obtain disclosure regarding certain aspects of the processing and obtain a copy of the information undergoing processing; (iv) you have the right to verify the accuracy of your information and ask for it to be updated or corrected; (v) you have the right, under certain circumstances, to restrict the processing of your information, in which case, we will not process your information for any purpose other than storing it; (vi) you have the right, under certain circumstances, to obtain the erasure of your Personal Information from us; (vii) you have the right to receive your information in a structured, commonly used and machine readable format and, if technically feasible, to have it transmitted to another controller without any hindrance. This provision is applicable provided that your information is processed by automated means and that the processing is based on your consent, on a contract which you are part of or on pre-contractual obligations thereof.</p>
<h3>The right to object to processing</h3>
<p>Where Personal Information is processed for the public interest, in the exercise of an official authority vested in us or for the purposes of the legitimate interests pursued by us, you may object to such processing by providing a ground related to your particular situation to justify the objection. You must know that, however, should your Personal Information be processed for direct marketing purposes, you can object to that processing at any time without providing any justification. To learn, whether we are processing Personal Information for direct marketing purposes, you may refer to the relevant sections of this document.</p>
<h3>How to exercise these rights</h3>
<p>Any requests to exercise User rights can be directed to the Owner through the contact details provided in this document. These requests can be exercised free of charge and will be addressed by the Owner as early as possible and always within one month.</p>
<h3>Privacy of children</h3>
<p>We do not knowingly collect any Personal Information from children under the age of 13. If you are under the age of 13, please do not submit any Personal Information through our Website or Service. We encourage parents and legal guardians to monitor their children's Internet usage and to help enforce this Policy by instructing their children never to provide Personal Information through our Website or Service without their permission. If you have reason to believe that a child under the age of 13 has provided Personal Information to us through our Website or Service, please contact us. You must also be at least 16 years of age to consent to the processing of your personal data in your country (in some countries we may allow your parent or guardian to do so on your behalf).</p>
<h3>Newsletters</h3>
<p>We offer electronic newsletters to which you may voluntarily subscribe at any time. You may choose to stop receiving our newsletter or marketing emails by following the unsubscribe instructions included in these emails or by contacting us.</p>
<h3>Cookies</h3>
<p>The Website uses "cookies" to help personalize your online experience. A cookie is a text file that is placed on your hard disk by a web page server. Cookies cannot be used to run programs or deliver viruses to your computer. Cookies are uniquely assigned to you, and can only be read by a web server in the domain that issued the cookie to you. We may use cookies to collect, store, and track information for statistical purposes to operate our Website and Services. You have the ability to accept or decline cookies. Most web browsers automatically accept cookies, but you can usually modify your browser setting to decline cookies if you prefer. To learn more about cookies and how to manage them, visit internetcookies.org In addition to using cookies and related technologies as described above, we also may permit certain third-party companies to help us tailor advertising that we think may be of interest to users and to collect and use other data about user activities on the Website. These companies may deliver ads that might also place cookies and otherwise track user behavior.</p>
<h3>Do Not Track signals</h3>
<p>Some browsers incorporate a Do Not Track feature that signals to websites you visit that you do not want to have your online activity tracked. Tracking is not the same as using or collecting information in connection with a website. For these purposes, tracking refers to collecting personally identifiable information from consumers who use or visit a website or online service as they move across different websites over time. How browsers communicate the Do Not Track signal is not yet uniform. As a result, this Website is not yet set up to interpret or respond to Do Not Track signals communicated by your browser. Even so, as described in more detail throughout this Policy, we limit our use and collection of your personal information.</p>
<h3>Information security</h3>
<p>We secure information you provide on computer servers in a controlled, secure environment, protected from unauthorized access, use, or disclosure. We maintain reasonable administrative, technical, and physical safeguards in an effort to protect against unauthorized access, use, modification, and disclosure of Personal Information in its control and custody. However, no data transmission over the Internet or wireless network can be guaranteed. Therefore, while we strive to protect your Personal Information, you acknowledge that (i) there are security and privacy limitations of the Internet which are beyond our control; (ii) the security, integrity, and privacy of any and all information and data exchanged between you and our Website cannot be guaranteed; and (iii) any such information and data may be viewed or tampered with in transit by a third-party, despite best efforts.</p>
<h3>Data breach</h3>
<p>In the event we become aware that the security of the Website has been compromised or users Personal Information has been disclosed to unrelated third parties as a result of external activity, including, but not limited to, security attacks or fraud, we reserve the right to take reasonably appropriate measures, including, but not limited to, investigation and reporting, as well as notification to and cooperation with law enforcement authorities. In the event of a data breach, we will make reasonable efforts to notify affected individuals if we believe that there is a reasonable risk of harm to the user as a result of the breach or if notice is otherwise required by law. When we do, we will post a notice on the Website.</p>
<h3>Legal disclosure</h3>
<p>We will disclose any information we collect, use or receive if required or permitted by law, such as to comply with a subpoena, or similar legal process, and when we believe in good faith that disclosure is necessary to protect our rights, protect your safety or the safety of others, investigate fraud, or respond to a government request. In the event we go through a business transition, such as a merger or acquisition by another company, or sale of all or a portion of its assets, your user account, and personal data will likely be among the assets transferred.</p>
<h3>Changes and amendments</h3>
<p>We reserve the right to modify this Policy relating to the Website or Services at any time, effective upon posting of an updated version of this Policy on the Website. When we do we will post a notification on the main page of our Website. Continued use of the Website after any such changes shall constitute your consent to such changes.</p>
<h3>Acceptance of this policy</h3>
<p>You acknowledge that you have read this Policy and agree to all its terms and conditions. By using the Website or its Services you agree to be bound by this Policy. If you do not agree to abide by the terms of this Policy, you are not authorized to use or access the Website and its Services.</p>
<h3>Contacting us</h3>
<p>If you have any questions about this Policy, please <a href="/contact">contact us</a>.</p>
                
            "#.to_owned()
        }
        Some("terms-and-conditions") => {
            r#"
            
            <h1>Terms and Conditions</h1>
<p>Welcome to UrgentCare Dental. These terms apply to all patients receiving treatment at our Manchester and Leeds locations.</p>
<h3>Appointment Deposits and Cancellations</h3>
<p>When you book an appointment with us, we may request a deposit to secure your slot. This deposit goes toward your treatment cost, but here's what you need to know about our cancellation policy.</p>
<p><strong>If you've paid a deposit for your next appointment and don't turn up, you'll lose that deposit.</strong> We understand emergencies happen, which is why we ask that you give us at least 24 hours notice if you need to cancel or reschedule. This lets us offer that valuable appointment slot to another patient in urgent need of care.</p>
<p>For same-day emergency appointments, we typically require full payment at the time of booking. These fees are non-refundable due to the immediate nature of the service we're providing.</p>
<h3>Payment Terms</h3>
<p>All treatment fees are due at the time of service unless you've arranged a payment plan with us in advance. We accept cash, card payments, and approved financing options for treatments over £500.</p>
<p>Our pricing is transparent and we'll always provide a full treatment estimate before beginning any work. For complex cases, the final cost may vary slightly from the initial estimate if additional treatment becomes necessary - we'll discuss this with you before proceeding.</p>
<h3>Payment Plans and Financing</h3>
<p>We offer 0% APR financing for qualifying patients on treatments over £500. Terms and conditions for financing are provided separately through our finance partners. Monthly payment plans are also available for our regular patients - speak to reception about setting one up.</p>
<h3>Treatment Consent and Clinical Decisions</h3>
<p>Before any treatment begins, we'll explain what needs to be done and obtain your informed consent. Our clinical team makes treatment recommendations based on professional judgment and current best practices in dentistry.</p>
<p>Sometimes during treatment, we may discover additional issues that need addressing. We'll always discuss these with you and get your consent before proceeding with any work beyond what was initially agreed.</p>
<h3>Emergency Treatment Priority</h3>
<p>As an emergency-focused dental service, we prioritize patients based on clinical need. While we aim to see all emergency patients on the same day, those with severe pain, trauma, or infection will be seen first.</p>
<p>For routine appointments, we operate on a first-come, first-served basis within each appointment category.</p>
<h3>Missed Appointments and Late Arrivals</h3>
<p>We understand that life happens, but missed appointments affect our ability to help other patients in need. If you're running more than 15 minutes late, we may need to reschedule your appointment to avoid delays for other patients.</p>
<p>Patients who repeatedly miss appointments without notice may be asked to pay in full before booking future appointments.</p>
<h3>Treatment Guarantees</h3>
<p>We stand behind our work. Fillings come with a 12-month guarantee, and crowns are guaranteed for 24 months, provided you maintain regular check-ups and follow our aftercare instructions.</p>
<p>This guarantee covers the quality of our work but doesn't cover damage from accidents, lack of proper care, or normal wear and tear. It also doesn't apply if you receive conflicting treatment elsewhere that affects our work.</p>
<h3>Patient Records and Privacy</h3>
<p>We maintain your dental records in accordance with NHS guidelines and data protection laws. Your information is kept strictly confidential and will only be shared with your consent or when legally required.</p>
<p>You have the right to request copies of your dental records. There may be a small administrative fee for providing these copies.</p>
<h3>Complaints Procedure</h3>
<p>We want every patient to be completely satisfied with their care. If you have any concerns about your treatment, please speak to our practice manager immediately. We aim to resolve all complaints within 10 working days.</p>
<p>If you're not satisfied with our response, you can escalate your complaint to:</p>
<ul>
  <li>
    <p>The Dental Complaints Service (for private treatment)</p>
  </li>
  <li>
    <p>NHS England (for any NHS treatment received)</p>
  </li>
</ul>
<h3>Changes to Terms</h3>
<p>We may update these terms from time to time to reflect changes in our services or legal requirements. The current version will always be available at our practices and on our website.</p>
<h3>NHS vs Private Treatment</h3>
<h3>We're a private dental practice. While NHS dentistry exists in principle, the reality is that new NHS patient spots are virtually impossible to find anywhere in the UK. We focus on providing accessible, fairly-priced private emergency dental care when you need it most.</h3>
<h3>Contact Information</h3>
<p><strong>Email</strong>: info@urgentcaredental.co.uk</p><p><strong>Website</strong>: urgentcaredental.co.uk</p>
<p>Last updated: September 2025</p>
                
            "#.to_owned()
        }
        Some("complaints") => {
            r#"
            
            <h1>Complaints Procedure</h1>
<h3>Complaints Handling</h3>
<p>We want our service to meet your expectations. If you have a concern or complaint about any aspect of our service, we want to identify how we can improve to ensure we meet your expectations in future. Our aim is to learn from any feedback we receive and improve our service.</p>
<p>We will deal with complaints courteously and promptly and aim to resolve the matter quickly.</p>
<h3>Making A Complaint</h3>
<p>If you wish to make a complaint or simply let us know how we could have done better, please contact our Complaints' Manager:</p>
<ul>
  <li>
    <p>In person</p>
  </li>
  <li>
    <p>By telephone</p>
  </li>
  <li>
    <p>Via email</p>
  </li>
  <li>
    <p>By letter</p>
  </li>
</ul>
<p>Our contact information is available <a href="./contact">on our contact page here</a>.</p>
<p>The Complaints' Manager will endeavour to be available during working hours. You may find it convenient to make an appointment with them to ensure they can dedicate sufficient time to meet.</p>
<p>If you contact the practice to make a complaint and the Complaints' Manager is not available, we will arrange a convenient time for them to contact you. We will ask you for brief details of your complaint so that the Complaints' Manager can gather any useful information before contacting you.</p>
<p>If the matter requires a more immediate response, we will arrange for a senior member of the team to deal with it.If your complaint is about your dental treatment or the fee charged, we will usually ask the dentist concerned to contact you, unless you do not want this.</p>
<p>We acknowledge all complaints in writing and enclose a copy of this code of practice as soon as possible, normally three working days.</p>
<h3>Investigating a complaint</h3>
<p>We will offer to discuss the complaint with you and will ask how you would like to be kept informed of developments - by telephone, letters or e-mail or by face-to-face meetings. We will let you know how we will deal with your complaint and the likely time that the investigation will take to complete. If you do not wish to discuss the complaint further, we will still let you know the expected timescale for completing the investigation.</p>
<p>We will investigate your complaint promptly (within 10 working days) or, if the issue is complex, within 6 months; and, as far as reasonably practicable, will let you know how our investigation is progressing.</p>
<p>When we have completed our investigation, we will provide you with a full written report, unless you have told us that you do not wish for further communication. The report will explain how we considered the complaint, the conclusions we reached for each part of your complaint, details of any remedial action we have taken and whether further action is needed.</p>
<h3>Records</h3>
<p>We keep proper and comprehensive records of any complaints that we receive and the action we have taken following investigation. We review these records regularly to ensure that we recognise our mistakes and take every opportunity to improve our service.</p>
<h3>If you are not satisfied</h3>
<p>If your complaint was about your dental treatment and you are not satisfied with the result of our investigation, you can take up the matter with a relevant external organisation:</p>
<h4>The Dental Complaints Service for complaints about private treatment</h4>
<p>Address:</p><p>Stephenson House<br>2 Cherry Orchard<br>Croydon<br>CR0 6BA</p>
<p>Phone: 0208 253 0800</p><p>Email: <a href="mailto:info@dentalcomplaints.org.uk">info@dentalcomplaints.org.uk</a></p>
                
            "#.to_owned()
        }
        Some("cookie-policy") => {
            r#"
            
            <h1>Cookie Policy</h1>
<p>This cookie policy ("Policy") describes what cookies are and how Us or We uses them on the website and any of its products or services (collectively, "Website" or "Services"). You should read this Policy so you can understand what type of cookies we use, the information we collect using cookies and how that information is used. It also describes the choices available to you regarding accepting or declining the use of cookies. For further information on how we use, store and keep your personal data secure, see our Privacy Policy.</p>
<h3>What are cookies?</h3>
<p>Cookies are small pieces of data stored in text files that are saved on your computer or other devices when websites are loaded in a browser. They are widely used to remember you and your preferences, either for a single visit (through a "session cookie") or for multiple repeat visits (using a "persistent cookie"). Session cookies are temporary cookies that are used during the course of your visit to the Website, and they expire when you close the web browser. Persistent cookies are used to remember your preferences within our Website and remain on your desktop or mobile device even after you close your browser or restart your computer. They ensure a consistent and efficient experience for you while visiting our Website or using our Services. Cookies may be set by the Website ("first-party cookies"), or by third parties, such as those who serve content or provide advertising or analytics services on the website ("third party cookies"). These third parties can recognize you when you visit our website and also when you visit certain other websites.</p>
<h3>Necessary cookies</h3>
<p>Necessary cookies allow us to offer you the best possible experience when accessing and navigating through our Website and using its features. For example, these cookies let us recognize that you have created an account and have logged into that account to access the content.</p>
<h3>Functionality cookies</h3>
<p>Functionality cookies let us operate the Website and our Services in accordance with the choices you make. For example, we will recognize your username and remember how you customized the Website and Services during future visits.</p>
<h3>Analytical cookies</h3>
<p>These cookies enable us and third-party services to collect aggregated data for statistical purposes on how our visitors use the Website. These cookies do not contain personal information such as names and email addresses and are used to help us improve your user experience of the Website.</p>
<h3>Social media cookies</h3>
<p>Third-party cookies from social media sites (such as Facebook, Twitter, etc) let us track social network users when they visit our Website, use our Services or share content, by using a tagging mechanism provided by those social networks. These cookies are also used for event tracking and remarketing purposes. Any data collected with these tags will be used in accordance with our and social networks' privacy policies. We will not collect or share any personally identifiable information from the user.</p>
<h3>Do we use web beacons or tracking pixels?</h3>
<p>Our emails may contain a "web beacon" (or "tracking pixel") to tell us whether our emails are opened and verify any clicks through to links or advertisements within the email. We may use this information for purposes including determining which of our emails are more interesting to users and to query whether users who do not open our emails wish to continue receiving them. The pixel will be deleted when you delete the email. If you do not wish the pixel to be downloaded to your device, you should read the email in plain text view or with images disabled.</p>
<h3>What are your cookie options?</h3>
<p>If you don't like the idea of cookies or certain types of cookies, you can change your browser's settings to delete cookies that have already been set and to not accept new cookies.</p>
<h3>Changes and amendments</h3>
<p>We reserve the right to modify this Policy relating to the Website or Services at any time, effective upon posting of an updated version of this Policy on the Website. When we do we will post a notification on the main page of our Website. Continued use of the Website after any such changes shall constitute your consent to such changes.</p>
<h3>Acceptance of this policy</h3>
<p>You acknowledge that you have read this Policy and agree to all its terms and conditions. By using the Website or its Services you agree to be bound by this Policy. If you do not agree to abide by the terms of this Policy, you are not authorized to use or access the Website and its Services.</p>
<h3>Contacting us</h3>
<p>If you have any questions about this Policy or our use of cookies, please <a href="./contact">contact us</a>.</p>
                
            "#.to_owned()
        }
        _ => String::new(),
    }
}

















