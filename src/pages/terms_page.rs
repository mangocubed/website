use dioxus::prelude::*;

use crate::{Routes, components::PageTitle};

#[component]
pub fn LinkToPrivacy() -> Element {
    rsx! {
        Link { to: Routes::privacy(), "Privacy Policy" }
    }
}

#[component]
pub fn TermsPage() -> Element {
    rsx! {
        PageTitle { "Terms of Service" }

        h1 { class: "h1", "Terms of Service" }

        p { class: "text-right font-bold", "Last updated on: September 16, 2025" }

        p {
            "These Terms of Service (\"Agreement\", or \"Terms\") are a legally binding agreement between Mango³ Group
            (\"Mango³\", \"we\", \"us\", \"our\") and"
        }

        p {
            "(i) Visitors: are any person that visit our website or App but do not have any Account on our Platform or
            have not subscribed to our Services; and"
        }

        p {
            "(ii) Subscribers: Subscribers are all Visitors who have a registered Account on our Platform and
            successfully completed Registration as per Section 4."
        }

        p {
            "Subscribers and Visitors are collectively referred to as \"Users\". All Users acknowledge and agree that
            their use of the Mango³ platform (\"Platform\") or accessing Mango³'s websites, Mango³'s application
            programming interface, or and Mango³'s mobile application (collectively the \"Services\") will be governed
            by this Agreement, our "

            LinkToPrivacy {}

            ", and any related terms. If Users are unsure as to the terms of this
            Agreement, they should not proceed further and contact us at "

            a { href: "mailto:support@mango3.app", "support@mango3.app" }

            ". User's use of our Services
            shall constitute User's acceptance of this Agreement and to all of the terms and conditions stated under
            this Agreement and our "

           LinkToPrivacy {}

            " referenced herein."
        }

        p {
            "PLEASE NOTE THAT THESE TERMS CONTAIN A BINDING AND MANDATORY ARBITRATION PROVISION AND CLASS ACTION/JURY
            TRIAL WAIVER PROVISION."
        }

        h2 { class: "h2", "1. INTRODUCTION" }

        p {
            "Mango³ provices a cloud storage platform with a focus on security and privacy. Users may access our
            Services through our website at "

            Link { to: Routes::home(), "https://mango3.app/" }

            ". Using Mango³, Subscribers can do the following:"
        }

        ul {
            li {
                "upload, access, store, manage, share, communicate, and download images (\"Files\") and data (\"Data\")"
           }
        }

        h2 { class: "h2", "2. PRIVACY POLICY" }

        p {
            "Our "

            LinkToPrivacy {}

            " describes how we handle the personal and business information Users provide to us when Users access or
            register for our Services. Users understand that through their use or accessing of our Services, they
            consent to the collection and use (as set forth in the "

            LinkToPrivacy {}

            ") of this information, including the transfer of this information to the US, and/or other countries for
            storage, processing and use by Mango³ and our affiliates."
        }

        h2 { class: "h2", "3. ELIGIBILITY TO ACCESS OUR WEBSITE/APP OR USE OUR SERVICES" }

        p {
            "(a) For Visitors: To be eligible to access our websites or apps, Visitors must meet the following criteria
            and represent and warrant that they: (a) are 18 years of age or older; (b) are not currently restricted
            from accessing our websites or apps, (c) are not our competitor, or are not using our Services for reasons
            that are in competition with us; (d) have full power and authority to enter into this Agreement and doing
            so will not violate any other agreement to which they are a party; (f) will not violate any of our rights,
            including intellectual property rights such as patent, copyright, and trademark rights; and (g) agree to
            provide at their cost all equipment, browser software, and internet access necessary to use our Services."
        }

        p {
            "(b) For Subscribers: To be eligible to create an Account on our Platform, as described in Section 4, or
            use our Services, Subscribers must represent and warrant that (a) they satisfy all conditions of Section
            3(a), (b) they will only maintain one registered account at any given time, (c) they are not prohibited
            from having an Account on our Platform"
        }

        h2 { class: "h2", "4. REGISTRATION FOR SUBSCRIBERS" }

        p {
            "a) Setting up an Account: A Visitor may only use our Platform and Services (\"Registration\") if they
            set up an account on our Platform (\"Account\"). For setting up an Account, they will be required to (a)
            enter their email address, and (b) set up a password (\"Password\") (collectively, the \"Account
            Information\"). Upon setting up an Account, the Visitor shall become a Subscriber."
        }
        
        p {
            "b) Financial Information and Subscription: Upon Registration, Subscribers will automatically be allowed
            to use the \"Free Plan\" as defined and detailed in Section 6 of these Terms. Subscribers may upgrade and
            change to a different plan (\"Subscription\"), as defined and detailed in Section 6 of these Terms, at any
            time during the Free Plan or at the Registration itself. To confirm a Subscription, Subscribers will have
            to provide us with certain financial information, as collected by our third party payment processors. By
            confirming Subscription, Subscribers will allow Mango³ to charge their card for the payments (including
            future payments), as authorized in accordance the Subscription Plan selected."
        }
        
        h2 { class: "h2", "5. SUBSCRIBERS' USE OF OUR PLATFORM" }
        
        p {
            "a) Subscriber's Files and Data: When Subscribers use our Services, they provide us with their Files and
            Data. These Terms don't give us any rights to a Subscriber's Files and Data except for the limited rights
            that enable us to offer the Services. We need a Subscriber's permission to host their Files and Data, back
            it up, and share it when a Subscriber asks us to. Subscribers represent and warrant that they own, or are
            authorized to use, any intellectual property in any Files or Data they store on, use, download, upload,
            share, access, transmit or otherwise make available to or from, our systems or using our services.
            Subscribers grant us a worldwide, royalty-free license to use, store, back-up, copy, transmit, distribute,
            communicate, modify and otherwise make available, their Files and Data, solely for the purposes of providing
            our Services and in accordance with these Terms and the "
            
            LinkToPrivacy {}
            
            "."
        }
        
        p {
            "b) Allowing Others to Access Data: If a Subscriber allows others (Authorized Persons) to access Subscriber's
            data (e.g. via any of the sharing features such as public links within Mango³), (i) the Authorized Person
            must accept these Terms, (ii) the Subscriber will be responsible for actions and omissions of the Authorized
            Persons while they are using our Services, and (iii) the Subscriber agrees to fully indemnify us for any
            claim, loss, damage, fine, costs (including our legal fees) and other liability if Authorized Persons breach
            any of these Terms."
        }
        
        p {
            "c) Subscriber's Undertakings: Subscribers (i) represent and warrant that all information they provide is
            true, accurate, current and complete, (ii) agree to maintain and update Account Information to keep it current,
            (iii) understand that they are responsible for maintaining the confidentiality of their Account Information,
            (iv) may not transfer or share Account Information with anyone unless they are Authorized Person,
            (v) understand that if they lose or misplace their Password, they will lose access to their Files and Data;
            (vi) shall maintain copies of all Data stored by them on our Platform, (vii) agree to immediately notify us
            upon becoming aware of any unauthorized use of their Account Information or Recovery Key, and (viii) understand
            that they are responsible for all activities that occur under their Account."
        }
        
        p {
            "d) Mango³'s Obligations: Mango³ reserves the right to take any and all action, as it deems necessary, regarding
            the security of our Service and a Subscriber's Account Information. Under no circumstances shall Mango³ be held
            liable to a Subscriber for any liabilities or damages resulting from or arising out of a Subscriber's use of the
            Platform, Account Information or a Subscriber's release of the Account Information to a third party. Mango³
            confirms that it will never send Subscribers emails asking for Subscriber's Password."
        }
        
        h2 { class: "h2", "6. PLANS AND PAYMENT" }
        
        p {
            "a) For Visitors: Access to our websites or apps is currently provided at no cost. If we change this in the
            future, we will communicate such modification to Visitors as per the procedure detailed under these Terms."
        }
        
        p { "b) For Subscribers:" }
        
        p {
            "(i) Free Plan: Subscribers can register to use our Services on a trial basis as per the terms of these Terms
            (\"Free Plan\"). Upon Registration, Subscribers will automatically be allowed to use the Free Plan."
        }
        
        p {
            "(ii) Paid Plans: Mango³ offers monthly and yearly paid Subscription plans (\"Paid Plans\"). Each Paid Plan allows
            a certain usage/storage limit, that is the amount of Files and Data Subscribers may upload and store on our Platform
            (\"Storage Limit\"). Subscribers may opt to subscribe to a Paid Plan at the time of Registration or at any time
            during the Free Plan (\"Purchase\" or \"Paid Subscription\"). Subscribers may increase their Storage Limit at any
            time during the duration of their ongoing Paid Plan by updating to a different Paid Plan and consenting to the such
            different Paid Plan's payment terms and Storage Limits. In the event that we modify the Paid Plans and their
            respective fees (\"Fees\") in the future, we will update that on the aforementioned link."
        }
    }
}
