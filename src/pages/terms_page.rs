use dioxus::prelude::*;

use crate::Routes;
use crate::components::PageTitle;

#[component]
fn LinkToPrivacy() -> Element {
    rsx! {
        Link { to: Routes::privacy(), "Privacy Policy" }
    }
}

#[component]
pub fn TermsPage() -> Element {
    rsx! {
        PageTitle { "Terms of Service" }

        h1 { class: "h1", "Terms of Service" }

        div {
            class: "prose prose-h2:text-xl max-w-none",
            p { class: "text-right", "Last updated on: " span { class: "font-bold", "September 17, 2025" } }

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

            h2 { "1. INTRODUCTION" }

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

            h2 { "2. PRIVACY POLICY" }

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

            h2 { "3. ELIGIBILITY TO ACCESS OUR WEBSITE/APP OR USE OUR SERVICES" }

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

            h2 { "4. REGISTRATION FOR SUBSCRIBERS" }

            p {
                "a) Setting up an Account: A Visitor may only use our Platform and Services (\"Registration\") if they
                set up an account on our Platform (\"Account\"). For setting up an Account, they will be required to (a)
                enter their email address, and (b) set up a password (\"Password\") (collectively, the \"Account
                Information\"). Upon setting up an Account, the Visitor shall become a Subscriber."
            }

            p {
                "b) Financial Information and Subscription: Upon Registration, Subscribers will automatically be allowed
                to use the \"Free Plan\" as defined and detailed in Section 6 of these Terms. Subscribers may upgrade
                and change to a different plan (\"Subscription\"), as defined and detailed in Section 6 of these Terms,
                at any time during the Free Plan or at the Registration itself. To confirm a Subscription, Subscribers
                will have to provide us with certain financial information, as collected by our third party payment
                processors. By confirming Subscription, Subscribers will allow Mango³ to charge their card for the
                payments (including future payments), as authorized in accordance the Subscription Plan selected."
            }

            h2 { "5. SUBSCRIBERS' USE OF OUR PLATFORM" }

            p {
                "a) Subscriber's Files and Data: When Subscribers use our Services, they provide us with their Files and
                Data. These Terms don't give us any rights to a Subscriber's Files and Data except for the limited
                rights that enable us to offer the Services. We need a Subscriber's permission to host their Files and
                Data, back it up, and share it when a Subscriber asks us to. Subscribers represent and warrant that they
                own, or are authorized to use, any intellectual property in any Files or Data they store on, use,
                download, upload, share, access, transmit or otherwise make available to or from, our systems or using
                our services. Subscribers grant us a worldwide, royalty-free license to use, store, back-up, copy,
                transmit, distribute, communicate, modify and otherwise make available, their Files and Data, solely for
                the purposes of providing our Services and in accordance with these Terms and the "

                LinkToPrivacy {}

                "."
            }

            p {
                "b) Allowing Others to Access Data: If a Subscriber allows others (Authorized Persons) to access
                Subscriber's data (e.g. via any of the sharing features such as public links within Mango³), (i) the
                Authorized Person must accept these Terms, (ii) the Subscriber will be responsible for actions and
                omissions of the Authorized Persons while they are using our Services, and (iii) the Subscriber agrees
                to fully indemnify us for any claim, loss, damage, fine, costs (including our legal fees) and other
                liability if Authorized Persons breach any of these Terms."
            }

            p {
                "c) Subscriber's Undertakings: Subscribers (i) represent and warrant that all information they provide
                is true, accurate, current and complete, (ii) agree to maintain and update Account Information to keep
                it current, (iii) understand that they are responsible for maintaining the confidentiality of their
                Account Information, (iv) may not transfer or share Account Information with anyone unless they are
                Authorized Person, (v) understand that if they lose or misplace their Password, they will lose access
                to their Files and Data; (vi) shall maintain copies of all Data stored by them on our Platform,
                (vii) agree to immediately notify us upon becoming aware of any unauthorized use of their Account
                Information or Recovery Key, and (viii) understand that they are responsible for all activities that
                occur under their Account."
            }

            p {
                "d) Mango³'s Obligations: Mango³ reserves the right to take any and all action, as it deems necessary,
                regarding the security of our Service and a Subscriber's Account Information. Under no circumstances
                shall Mango³ be held liable to a Subscriber for any liabilities or damages resulting from or arising
                out of a Subscriber's use of the Platform, Account Information or a Subscriber's release of the Account
                Information to a third party. Mango³ confirms that it will never send Subscribers emails asking for
                Subscriber's Password."
            }

            h2 { "6. PLANS AND PAYMENT" }

            p {
                "a) For Visitors: Access to our websites or apps is currently provided at no cost. If we change this in
                the future, we will communicate such modification to Visitors as per the procedure detailed under these
                Terms."
            }

            p { "b) For Subscribers:" }

            p {
                "(i) Free Plan: Subscribers can register to use our Services on a trial basis as per the terms of these
                Terms (\"Free Plan\"). Upon Registration, Subscribers will automatically be allowed to use the Free
                Plan."
            }

            p {
                "(ii) Paid Plans: Mango³ offers monthly and yearly paid Subscription plans (\"Paid Plans\"). Each Paid
                Plan allows a certain usage/storage limit, that is the amount of Files and Data Subscribers may upload
                and store on our Platform (\"Storage Limit\"). Subscribers may opt to subscribe to a Paid Plan at the
                time of Registration or at any time during the Free Plan (\"Purchase\" or \"Paid Subscription\").
                Subscribers may increase their Storage Limit at any time during the duration of their ongoing Paid Plan
                by updating to a different Paid Plan and consenting to the such different Paid Plan's payment terms and
                Storage Limits. In the event that we modify the Paid Plans and their respective fees (\"Fees\") in the
                future, we will update that on the aforementioned link."
            }

            p {
                "(iii) Fees: In accordance with the selected Paid Plan, the Fees will be billed monthly or annually.
                Subscribers will pay, if applicable, all applicable customs, duties, sales, use, value added or other
                taxes, federal, state or otherwise, however designated, which are levied or imposed by reason of the
                transactions contemplated by these Terms, excluding only taxes based on Mango³'s net income.
                Subscriptions of Paid Plans are recurring will renew indefinitely, either monthly or annually, based
                upon Subscriber's chosen Paid Plan, unless the Subscription is cancelled prior to a renewal date. For
                recurring Paid Plan Subscriptions established via our apps using in-app-purchase platforms, Subscribers
                should refer to their app store or play store account for details and terms of the Subscription. Any
                other recurring Paid Plan Subscriptions will renew on the same day of month as it was established,
                except in cases where the day is not available due to a short month, in which case the renewal date
                will be moved to the first day of the following month."
            }

            p {
                "(iv) Refunds: Unless otherwise provided by applicable laws or by a particular Service offer, all
                Purchases are final and non-refundable. If Subscribers believe that Mango³ has charged them in error,
                they must contact us within ninety (90) days of such charge. No refunds will be given for any charges
                more than ninety (90) days old. We reserve the right to issue refunds or credits at our sole discretion.
                If we issue a refund or credit, we are under no obligation to issue the same or similar refund in the
                future. This refund policy does not affect any statutory rights that may apply. IF SUBSCRIBERS HAVE MADE
                A PAYMENT BY MISTAKE AND HAVE NOT USED THE PAID PLAN SERVICES, SUBSCRIBERS MUST CONTACT "

                a { href: "mailto:support@mango3.app", "SUPPORT@MANGO3.APP" }

                " WITHIN 24 HOURS. This will be acknowledged promptly and answered within 7 days."
            }

            p {
                "(v) Exceeding Storage Limit: If Subscribers exceed their Storage Limit, Mango³ will stop backing up
                their Files and Data, and they will receive an email alerting them of the same. However, Subscriber's
                backed up Files will remain accessible for as long as they have an active subscription."
            }

            p {
                "(vi) Non-Payment: If at any time Subscribers do not make a payment to us when they are supposed to
                (including on Termination), we can suspend or terminate their use of the service. We may also
                additionally pursue any other rights or remedies we may have against such Subscribers."
            }

            h2 { "7. TERMINATION" }

            p {
                "a) For Visitors: We reserve the right to cease providing Visitors with access to all or part of our
                Services at any time for any reason, including, but not limited to, if we reasonably believe:
                (i) they have violated this Agreement or our ",

                LinkToPrivacy {}

                " (ii) they create risk or possible legal exposure for Mango³; or
                (iii) the provision of our Services to them is no longer commercially viable."
            }

            p {
                "b) For Subscribers:"
            }

            p {
                "(i) Termination by us: We reserve the right to suspend or terminate a Subscriber's Account and their
                license to use our Services for any reason, including, but not limited to, if we reasonably believe:
                (a) they have violated this Agreement or our "

                LinkToPrivacy {}

                ",
                (b) they create risk or possible legal exposure for Mango³;
                (c) the provision of our Services to them is no longer commercially viable;
                (d) they are using a free account, without an active paid subscription, that has been inactive for a
                consecutive period of 12 months. In case of inactivity, notices will be sent 30, 15, and 7 days in
                advance to the email address associated with the free account. To keep a free account active, a
                subscriber must log in or use one of our services (e.g., by viewing a photo or code) at least once
                every 12 months. If you face circumstances preventing activity, contact "

                a { href: "mailto:support@mango3.app", "Customer Support" }

                " to avoid account deletion."
            }

            p {
                "(ii) Cancellation and Termination by Subscribers: Subscribers can cancel their Subscription
                (\"Cancellation\") at any time by (i) cancelling directly through the relevant app store account; or
                (iii) selecting the option to cancel Subscription on the Subscriber's Account page. Subscribers can
                terminate their Account and thereby our Services by (i) writing an email to "

                a { href: "mailto:support@mango3.app", "support@mango3.app" }

                " (\"Termination\"). Any payments processed after an effective Cancellation or Termination will be
                promptly refunded by us."
            }

            p {
                "(iii) Effect of Termination: Subscribers should download all Data prior to Termination of Services.
                Upon Termination, we will schedule their uploaded Files and Data for deletion within the next thirty
                (30) days. In case of shared Files and Data, we may retain them to ensure access to Authorized Persons.
                We may also maintain Subscriber's email address, IP address and user agent information in a hashed form
                to prevent an abuse of Free Plans. If we suspend or terminate our Services to a Subscriber because they
                or their Authorized Persons have breached these Terms, we may, delete a Subscriber's Data immediately,
                deny them access to their Data, or keep it for evidential purposes during the suspension."
            }

            p {
                "c) For All Users: All sections, which by their nature and context are intended to survive the
                termination of this Agreement, will survive"
            }

            h2 { "8. SERVICE LICENSE FOR ALL USERS" }

            p {
                "Subject to compliance with the terms of this Agreement, we grant Users a limited, non-exclusive,
                revocable, non-transferable, non-licensable, non-sublicensable license to access and use our websites
                or apps to: (a) download, install, and use our app in accordance with this Agreement on any mobile
                device owned or otherwise controlled by Users, and (b) access, stream, download, and use on User's
                mobile device our websites or apps and content made available in or otherwise accessible through our
                websites or apps, strictly in accordance with this Agreement."
            }

            h2 { "9. SERVICE RESTRICTIONS FOR ALL USERS" }

            p {
                "Users agree that our Platform, including but not limited to the websites, apps, graphics, trademarks,
                and editorial content, contains proprietary content, information, and material, which are owned by
                Mango³ and/or our licensors, including our customers, brands and agencies, and are protected by
                applicable intellectual property and other laws, including but not limited to copyright. Users agree
                that Users will not use such proprietary content, information or materials other than for Users'
                permitted use of our Platform or in any manner that is inconsistent with the terms contained in this
                Agreement."
            }

            h2 { "10. RESERVATION OF RIGHTS FOR ALL USERS" }

            p {
                "Except to the extent necessary to access and use our websites or apps, nothing in this Agreement grants
                any title or ownership interest in or to any copyrights, patents, trademarks, trade secrets or other
                proprietary rights in or relating to our websites or apps, whether expressly, by implication, estoppel,
                or otherwise. Mango³ and its licensors and service providers reserve and will retain their entire right,
                title, and interest in and to our Services, including all copyrights, trademarks, and other intellectual
                property rights therein or relating thereto, except as expressly granted to Users in this Agreement."
            }

            h2 { "11. REQUIRED CONDUCT" }

            p { "a) For Visitors: As a condition to access our websites or apps, Visitors agree to:" }

            p {
                "i. Comply with all applicable laws, including, without limitation, tax laws, export control laws and
                regulatory requirements when using our Services and with respect to any data Subscribers upload, access
                or share using our services.;"
            }

            p {
                "ii. Review our " LinkToPrivacy {} " and comply with the same;"
            }

            p { "iii. Make sure that their internet connection is adequate;" }

            p {
                "iv. To not duplicate, license, sublicense, publish, broadcast, transmit, distribute, perform, display,
                sell, rebrand, otherwise transfer or commercially exploit our Services, other than as permitted by
                applicable open source licenses;"
            }

            p {
                "v. To not reverse engineer, modify, decompile, disassemble, decipher, capture screen shots, for any
                underlying intellectual property used to provide our websites or apps, or any part thereof, other than
                as permitted by applicable open source licenses;"
            }

            p { "vi. To not resell or otherwise supply our Services to anyone else without our prior written consent;" }

            p {
                "vii. To not imply or state, directly or indirectly, that they are affiliated with or endorsed by Mango³
                unless they have entered into a written agreement with us;"
            }

            p {
                "viii. To not adapt, modify, or create derivative works based on our Services or technology underlying
                our Services, or other users' content, in whole or in part, other than as permitted by applicable open
                source licenses;"
            }

            p {
                "ix. To not rent, lease, loan, trade, sell/re-sell access to our Services or any information therein,
                or the equivalent, in whole or part;"
            }

            p {
                "x. To not access, reload, or \"refresh\" or make any other request to transactional servers that are
                beyond generally accepted usage of web-based applications;"
            }

            p {
                "xi. To not use manual or automated software, devices, scripts robots, other means or processes to
                \"scrape\", \"crawl\" or \"spider\" any web pages contained in the website;"
            }

            p {
                "xii. To not engage in \"framing\", \"mirroring\", or otherwise simulating the appearance or function
                of our website;"
            }

            p {
                "xiii. To not attempt to or actually access our websites or apps by any means other than through the
                interface provided by Mango³;"
            }

            p {
                "xiv. To not attempt to or actually override any security component included in or underlying our
                Services;"
            }

            p {
                "xv. To not engage in any action that interferes with the proper working of or places an unreasonable
                load on our infrastructure, including but not limited to unsolicited communications, attempts to gain
                unauthorized access, or transmission or activation of computer viruses;"
            }

            p {
                "xvi. To not remove any copyright, trademark, or other proprietary rights notices contained in or on
                our Services, including those of both Mango³ or any of our licensors;"
            }

            p {
                "xvii. To not exploit our Services in any unauthorized way whatsoever, including but not limited to,
                using our websites or apps to transmit any computer viruses, worms, Trojan horses or other malware, or
                by trespassing or burdening network capacity; and"
            }

            p {
                "xviii. To not to use our Services in any manner to harass, abuse, stalk, threaten, defame or otherwise
                infringe or violate the rights of any other party."
            }

            p { "b) For Subscribers: As a condition to use our Services, Subscribers agree to:" }

            p { "i. Comply with Section 11 (a);" }

            p {
                "ii. Provide accurate information (including correct contact and any billing details) to Mango³ and
                update from time to time as may be necessary;"
            }

            p { "iii. Review and comply with notices sent by Mango³, if any, concerning our Services;" }

            p {
                "iv. Make sure their Password is strong, secure, not used by Subscribers on other sites and
                confidential;"
            }

            p {
                "v. Inform us if they think or know someone else has used their Password, Recovery Key, or there has
                been any other security breach;"
            }

            p {
                "vi. To not utilize information, content or any data they view on and/or obtain from our Services to
                provide any service that is competitive with us;"
            }

            p { "vii. To not infringe anyone else's intellectual property or other rights in any data;" }

            p { "viii. To not open multiple free accounts or register for multiple Free Plans;" }

            p {
                "ix. To not adapt, modify, or create derivative works based on other Users' content, in whole or in
                part;"
            }

            p {
                "x. To not upload or promote any content that is harmful, offensive, illegal, unlawful, discriminatory,
                dangerous, profane, or abusive."
            }

            h2 { "12. INDEMNIFICATION BY ALL USERS" }

            p {
                "Users agree to indemnify, defend, and hold Mango³ and our officers, employees, managers, directors,
                customers, and agents (the \"Indemnitees\") harmless from and against any and all costs, liabilities,
                losses and expenses (including but not limited to reasonable attorneys' fees) resulting from any claim,
                suit, action, demand or proceeding brought by any third party against Mango³ and our Indemnitees arising
                from any of the following: (i) a breach of this Agreement; (ii) the negligence, fraud, or willful
                misconduct of Users or their employees, agents, or contractors; (iii) incorrect information provided by
                Users in their Account or elsewhere; or (iv) a failure by Users or their employees, agents, contractors
                or invitees to comply with applicable laws and regulations."
            }

            h2 { "13. DISCLAIMERS TO ALL USERS" }

            p {
                "Users' access to and use of our Services or any content are at their own risk. Users understand and\
                agree that our Services are provided to Users on an \"AS IS\" and \"AS AVAILABLE\" basis. Without
                limiting the foregoing, to the maximum extent permitted under applicable law, WE DISCLAIM ALL
                WARRANTIES AND CONDITIONS, WHETHER EXPRESS OR IMPLIED, OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
                PURPOSE, AND NON-INFRINGEMENT. We make no warranty or representation and disclaim all responsibility
                and liability for:"
            }

            p {
                "(i) the completeness, accuracy, availability, timeliness, security or reliability of our websites or
                apps or any content;"
            }

            p {
                "(ii) any harm to Users' computer system, loss of data, or other harm that results from their access to
                or use of our Services or any content;"
            }

            p {
                "(iii) the deletion of, or the failure to store or to transmit, any content and other communications
                maintained by our websites or apps;"
            }

            p { "(iv) that our Services will be available on an uninterrupted, secure, or error-free basis;" }

            p { "(v) that the Services should be available 24 hours a day, seven days a week;" }

            p { "(vi) devices or equipment that we do not own or have not given Users;" }

            p {
                "(vii) any actions or omissions of other people which disrupt access to our Services including the (a)
                content and nature of any Data that Subscribers upload, access or share, (b) content of other people's
                websites even if a link to their website is included on our website or our apps."
            }

            p {
                "Mango³ does not review, verify, revise, endorse, or otherwise approve any content created or posted by
                our Users, and communicated to other users or third parties via our Platform, but Mango³ will remove
                content that violates any laws or this agreement. Under no circumstances will Mango³ be liable in any
                way for any content created or posted by Users. The content is solely created by our Users, and MANGO³
                SPECIFICALLY DISCLAIMS ANY AND ALL ROLE WHATSOEVER WITH RESPECT TO THE CREATION OR POSTING OF SUCH
                CONTENT."
            }

            h2 { "14. LIMITATION OF LIABILITY TOWARDS ALL USERS" }

            p {
                "All Users acknowledge and agree that, in no event will Mango³ be liable to Users or any third party
                for any indirect, punitive, exemplary, incidental, special, or consequential damages whether in
                contract, tort (including negligence), or otherwise arising out of this Agreement, or the use of, or
                the inability to use, our Services, including, without limitation, any information made available
                through our websites or apps pursuant to this Agreement. In the event the foregoing limitation of
                liability is determined by a court of competent jurisdiction to be unenforceable, then the maximum
                liability for all claims of every kind will not exceed one times (1x) the payments received under
                User's most recent Paid Plan Fee. The foregoing limitation of liability will cover, without limitation,
                any technical malfunction, computer error or loss of data, and any other injury arising from the use of
                our Services. Some jurisdictions do not allow the exclusion of certain warranties or the limitation or
                exclusion of liability for incidental or consequential damages. To the extent that Mango³ may not
                disclaim any implied warranty or limit its liabilities, the scope and duration of such warranty and the
                extent of Mango³'s liability will be the minimum permitted under applicable law."
            }

            h2 { "15. COPYRIGHT INFRINGEMENT NOTICE AND TAKEDOWN PROCEDURES FOR ALL USERS" }

            p {
                "a) DMCA Takedown Notice: If Users believe that any content on our websites or apps violates their
                copyright, and Users wish to have the allegedly infringing material removed, the following information
                in the form of a written notification (pursuant to the Digital Millennium Copyright Act of 1998
                (\"DMCA Takedown Notice\")) must be provided to our designated Copyright Agent."
            }

            p { "i. User's physical or electronic signature;" }

            p { "ii. Identification of the copyrighted work(s) that Users claim to have been infringed;" }

            p {
                "iii. Identification of the material on our websites or apps that Users claim is infringing and that
                Users request us to remove;"
            }

            p { "iv. Sufficient information to permit us to locate and access such material;" }

            p { "v. User's address, telephone number, and email address;" }

            p {
                "vi. A statement that Users have a good faith belief that use of the objectionable material is not
                authorized by the copyright owner, its agent, or under the law; and"
            }

            p {
                "vii. A statement that the information in the notification is accurate, and under penalty of perjury,
                that Users are either the owner of the copyright that has allegedly been infringed or that Users are
                authorized to act on behalf of the copyright owner."
            }

            p {
                "b) User's Obligations: We respect the copyright of others and require that users of our Services
                comply with copyright laws. Users are strictly prohibited from using our Services to infringe
                copyright. Users may not upload, download, store, share, access, display, stream, distribute, e-mail,
                link to, communicate, transmit, or otherwise make available any Files, Data, or content that infringes
                any copyright or other proprietary rights of any person or entity."
            }

            p {
                "c) Our Rights: We reserve the right to remove data alleged to be infringing without prior notice, at
                our sole discretion, and without liability to Users. In appropriate circumstances, we will also
                terminate User's account if we consider Users to be a bad actor or repeat infringer."
            }

            p {
                "e) DMCA Counter-Notices: A counter notification is a legal request for Mango³ to reinstate a File or
                Data that was taken down for alleged copyright infringement."
            }

            ol {
                li {
                    "Users may file a counter-notice if Users believe that access to a File Users have uploaded has
                    been wrongly or mistakenly disabled because it was the subject of an incorrect takedown notice.
                    Users should only do so if Users are confident that no other party owns copyright in the material,
                    or Users have rights to store the material and, if Users are sharing it, that Users have the right
                    to do so."
                }

                li {
                    p { "Users understand that:" }

                    p {
                        "i. When we receive User's counter-notice, we pass it, including User's address and other
                        contact information, to the party who issued the original takedown notice. By submitting User's
                        counter-notice Users authorize us to do so."
                    }

                    p {
                        "ii. Filing a counter-notice may lead to legal proceedings between Users and the complaining
                        party. Submitting a counter notice can have real legal consequences. If the complaining party
                        disagrees that their DMCA Takedown Notice was mistaken, they might decide to file a lawsuit
                        against Users to keep the content disabled. Users should conduct a thorough investigation into
                        the allegations made in the DMCA Takedown Notice and talk to a lawyer before submitting a
                        counter notice."
                    }

                    p {
                        "iii. There may be adverse legal consequences if Users make a false or bad faith allegation by
                        using this process. If, when using this counter-notice process, Users make a false or bad faith
                        allegation or otherwise breach these Terms or any of our policies and that causes us any loss,
                        costs, damages or other liability, we reserve the right to claim for and recover from Users
                        that loss, those costs (including full legal costs on a solicitor-client basis), damages and
                        other liability, by deduction from any balance in our account."
                    }

                    p {
                        "iv. We provide this counter-notice process voluntarily for the purposes of all applicable
                        copyright takedown regimes, but in doing so, we do not submit to any jurisdiction, law,
                        tribunal or court other than those as set out in these Terms. We may amend, suspend or
                        withdraw this counter-notice process at any time, provided that any counter-notices in train
                        at that time shall continue to be processed."
                    }

                    p {
                        "v. In order to file a counter notice, Users must have \"a good faith belief that the material
                        was removed or disabled as a result of mistake or misidentification of the material to be
                        removed or disabled\". Whether Users decide to explain why Users believe there was a mistake is
                        up to Users and User's lawyer, but Users do need to identify a mistake before Users submit a
                        counter notice."
                    }

                    p {
                        "vi. After redacting personal information, we may publish redacted versions of complete and
                        actionable counter notices on our Website."
                    }

                    p {
                        "vii. Mango³ isn't the judge. Mango³ exercises little discretion in this process other than
                        determining whether the notices meet the minimum requirements of the DMCA. It is up to the
                        parties (and their lawyers) to evaluate the merit of their claims, bearing in mind that
                        notices must be made under penalty of perjury."
                    }
                }

                li {
                    "By filing a counter-notice, Users are deemed to have accepted the above Terms. If Users do not
                    accept the above terms, do not file a counter-notice."
                }

                li {
                    p {
                        "To file a counter-notice with us, Users must provide a written communication to "

                        a { href: "mailto:support@mango3.app", "suopport@mango3.app" }

                        " that includes substantially the following:"
                    }

                    p {
                        "i.  Identification of the specific URL(s) of material that has been removed or to which access
                        has been disabled."
                    }
                    p {
                        "ii. User's full name, address, telephone number, email address and the name within User's
                        Mango³ account."
                    }
                    p {
                        "iii. The statement: \"I have a good faith belief that the material was removed or disabled as
                        a result of a mistake or misidentification of the material to be removed or disabled.\""
                    }
                    p {
                        "iv. The reasons for that good faith belief, sufficient to explain the mistake or
                        misidentification to the person who filed the original takedown notice."
                    }

                    p { "v. A scanned physical signature or usual signoff in an email." }
                }
                li {
                    "We will only accept a counter-notification directly from the User from whose account a folder or
                    file has been disabled. Counter-notifications must be submitted from the email address associated
                    with the Mango³ Account."
                }

                li {
                    "If we do not receive any further communication from or on behalf of the person who originally
                    submitted the takedown notice, or any communication we do receive does not in our sole opinion
                    adequately justify the original takedown notice, we may, but shall not be obliged to, reinstate the
                    material in approximately 10-14 days provided we have no reason to believe that the material
                    infringes copyright."
                }

                li {
                    "Nothing in this counter-notice section prejudices our right to remove or disable access to any
                    material at any time, for any reason or no reason."
                }
            }

            p {
                "f) Takedown of Objectionable Content: We feel privacy is a human right - that is why we are building
                our Services. But we are not above the law. We will comply with reasonable takedown requests for
                objectionable content, and will promptly inform Users of any requests received against them where
                appropriate and practicable under applicable law. We also reserve the right to take any other action we
                deem fit, including disabling accounts where objectionable content is discovered, to the extent
                permitted by law."
            }

            h2 { "16. ASSIGNMENT BY ANY USER" }

            p {
                "This Agreement is only for Users' benefit. Users shall have no right to assign this Agreement or any
                benefits or obligation hereunder to any other party or legal entity. Any attempted assignment shall be
                void."
            }

            h2 { "17. ANTI-BRIBERY AND EXPORT COMPLIANCE FOR ALL USERS" }

            p {
                "Users agree not to promote, approach, use, distribute, transfer, provide, sub-license, share with, or
                otherwise offer our Services in violation of any laws or this Agreement, including, including
                anti-corruption statutes. Without limiting the foregoing, Users will not knowingly directly or
                indirectly export, re-export, transfer, make available or release (collectively, \"Export\") our
                Services, Services to any destination, person, entity or end-use prohibited or restricted under the
                applicable law without prior US government authorization to the extent required by the applicable
                export control regulations, including without limitation, to any parties listed on any of the denied
                parties lists or specially designated nationals lists to the extent required by the applicable
                regulations."
            }

            h2 { "18. MODIFICATIONS" }

            p {
                "We reserve the right, at our sole discretion, to change or modify this Agreement at any time. Your
                continued use of our Services confirms your acceptance of our updated Agreement."
            }

            h4 {"19. RELATIONSHIP OF MANGO³ WITH USERS" }

            p {
                "The parties hereto are independent contractors, and nothing contained herein shall be interpreted as
                creating any relationship other than that of independent contracting parties. The parties shall not be
                construed as being partners, joint ventures, shareholders, employer/employee, or agent/servant. The
                User has no power or authority to bind Mango³ to any obligation, agreement, debt or liability. The User
                shall not hold itself out as an agent or representative of Mango³."
            }

            h2 { "20. DISPUTE RESOLUTION AND ARBITRATION FOR ALL USERS" }

            p { "a.  Binding Arbitration" }

            p {
                "Except for any disputes, claims, suits, actions, causes of action, demands or proceedings
                (collectively, \"Disputes\") in which either party seeks to bring an individual action in small claims
                court or seeks injunctive or other equitable relief for the alleged unlawful use of intellectual
                property, including, without limitation, copyrights, trademarks, trade names, logos, trade secrets or
                patents, Users and Mango³ agree (a) to waive Users' and Mango³'s respective rights to have any and all
                Disputes arising from or related to this Agreement, use of our Services, resolved in a court, and
                (b) to waive Users' and Mango³'s respective rights to a jury trial. Instead, Users and Mango³ agree to
                arbitrate Disputes through binding arbitration (which is the referral of a Dispute to one or more
                persons charged with reviewing the Dispute and making a final and binding determination to resolve it
                instead of having the Dispute decided by a judge or a jury in court)."
            }

            p { "b.  No Class Arbitrations, Class Actions or Representative Actions" }

            p {
                "Users and Mango³ agree that any Dispute arising out of or related to these Terms or use or access of our
                Services is personal to Users and Mango³ and that such Dispute will be resolved solely through individual
                arbitration and will not be brought as a class arbitration, class action or any other type of
                representative proceeding. Users and Mango³ agree that there will be no class arbitration or arbitration
                in which an individual attempts to resolve a Dispute as a representative of another individual or group
                of individuals. Further, Users and Mango³ agree that a Dispute cannot be brought as a class or other type
                of representative action, whether within or outside of arbitration, or on behalf of any other
                individual or group of individuals."
            }

            p { "c.  Notice; Informal Dispute Resolution" }

            p {
                "Users and Mango³ agree that each party will notify the other party in writing of any arbitral or small
                claims Dispute within thirty (30) days of the date it arises, so that the parties can attempt in good
                faith to resolve the Dispute informally. Users' notice must include (a) Users' name, postal address,
                telephone number, the email address Users use or used for User's Mango³ account and, if different, an
                email address at which Users can be contacted, (b) a description in reasonable detail of the nature or
                basis of the Dispute, and (c) the specific relief that Users are seeking. Our notice to Users will be
                sent electronically in accordance with this Agreement and will include (x) our name, postal address,
                telephone number and an email address at which we can be contacted with respect to the Dispute, (y) a
                description in reasonable detail of the nature or basis of the Dispute, and (z) the specific relief
                that we are seeking. If Users and Mango³ cannot agree how to resolve the Dispute within thirty (30) days
                after the date notice is received by the applicable party, then either Users or Mango³ may, as
                appropriate and in accordance with this Section, commence an arbitration proceeding."
            }

            p { "d.  Process" }

            p {
                "EXCEPT FOR DISPUTES IN WHICH EITHER PARTY SEEKS TO BRING AN INDIVIDUAL ACTION IN SMALL CLAIMS COURT OR
                SEEKS INJUNCTIVE OR OTHER EQUITABLE RELIEF FOR THE ALLEGED UNLAWFUL USE OF INTELLECTUAL PROPERTY,
                INCLUDING, WITHOUT LIMITATION, COPYRIGHTS, TRADEMARKS, TRADE NAMES, LOGOS, TRADE SECRETS OR PATENTS,
                USERS AND MANGO³ AGREE THAT ANY DISPUTE MUST BE COMMENCED OR FILED BY USERS OR ENTE WITHIN (1) YEAR OF
                THE DATE THE DISPUTE AROSE, OTHERWISE THE UNDERLYING CLAIM IS PERMANENTLY BARRED (WHICH MEANS THAT
                USERS AND MANGO³ WILL NO LONGER HAVE THE RIGHT TO ASSERT SUCH CLAIM REGARDING THE DISPUTE)."
            }

            p { "e.  Severability" }

            p {
                "If any term, clause or provision of this Section is held invalid or unenforceable, it will be so held
                to the minimum extent required by law, and all other terms, clauses and provisions of this Section will
                remain valid and enforceable. Further, the waivers set forth herein are severable from the other
                provisions of this Agreement and will remain valid and enforceable, except as prohibited by applicable
                law."
            }

            h2 { "21. MISCELLANEOUS" }

            p {
                "This Agreement along with our "

                LinkToPrivacy {}

                " constitutes the entire agreement between Users and Mango³ and supersedes any prior agreements between
                Users and Mango³ with respect to the subject matter herein. Our failure to exercise or enforce any
                right or provision of this Agreement will not constitute a waiver of such right or provision. If
                any provision of this Agreement is found by a court of competent jurisdiction to be invalid, we both
                nevertheless agree that the court should endeavor to give effect to our intentions as reflected in this
                provision, and the other provisions of this Agreement to remain in full force and effect. Users agree
                that regardless of any statute or law to the contrary, any claim or cause of action arising out of or
                related to use of our Services or this Agreement must be filed within one (1) year after such claim or
                cause of action arose or be forever barred. A party's failure to act with respect to a breach by the
                other party does not constitute a waiver of the party's right to act with respect to subsequent or
                similar breaches. All the sections intended to survive the termination of this Agreement shall survive.
                The section titles in this Agreement are for convenience only and have no legal or contractual effect.
                Any notices to Users shall be provided to Users through our Services or given to Users via the email
                address Users provide to Mango³."
            }
        }
    }
}
