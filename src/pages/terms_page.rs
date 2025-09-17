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
            li { "upload, access, store, manage and download images" }
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
            ""
        }
    }
}
