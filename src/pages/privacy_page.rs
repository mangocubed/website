use dioxus::prelude::*;

use sdk::app::components::{H1, PageTitle};

use crate::Routes;

#[component]
fn LinkToTerms() -> Element {
    rsx! {
        Link { to: Routes::terms(), "Terms of Service" }
    }
}

#[component]
pub fn PrivacyPage() -> Element {
    rsx! {
        PageTitle { "Privacy Policy" }

        H1 {  "Privacy Policy" }

        div { class: "prose prose-h2:text-xl max-w-none",
            p { class: "text-right",
                "Last updated on: "
                span { class: "font-bold", "September 17, 2025" }
            }

            p {
                "Mango³ Group (\"Mango³\", \"we\", \"us\", \"our\") respects the privacy of its Users (\"User\",
                \"your\", or \"you\"). This Privacy Policy (the \"Privacy Policy\") explains how we collect, use,
                disclose, and safeguard your information when you use Mango³'s platform, websites or apps (together
                \"Services\"). This Privacy Policy is applicable to our Services offered for sale to the public."
            }

            p {
                "Please read this privacy policy carefully to understand our policies and practices regarding your
                information and how we will treat it. By accessing or using our Services, you agree to accept all the
                terms contained in this Privacy Policy and acknowledge and agree with the practices described herein."
            }

            p {
                "If you do not agree with the terms of this privacy policy, please do not access and use our Services."
            }

            p { "We do not sell your personal information, nor do we intend to do so." }

            h2 { "1. INTRODUCTION" }

            p {
                "Mango³ provides cloud based mobile and desktop storage apps with a focus on security and privacy. We
                have open source apps across platforms that securely backup all your images, etc. (\"File(s)\"). We
                preserve them at multiple storage locations, and only you can decrypt them. We collect and store only
                the bare minimum amount of information necessary to fulfill our role as a service provider. All of your
                files and the metadata related to those files are stored encrypted, and only you hold the decryption
                keys. We collect no app usage metrics and crash reports (if any) are anonymized."
            }

            h2 { "2. TERRITORIAL RESTRICTION" }

            p {
                "Our Services are available for use and download globally. Please ensure compliance with all local laws
                prior to using our Services."
            }

            h2 { "3. WHAT INFORMATION DO WE COLLECT?" }

            p {
                "a) Information you Provide: At the time of registration, or through your use of our Services, you will
                provide us with"
            }

            ol {
                li { "Your email address;" }

                li { "Referral details including referrers and people you have referred;" }

                li { "Email addresses you choose to share your Files with;" }

                li { "Our Communications with you and records or copies of such communications;" }

                li {
                    "Other personal information you provide to us for support purposes, bug reports, newsletters,
                    surveys, sweepstakes, product feedback, or via forms."
                }
            }

            p { "b) Information we automatically collect:" }

            ol {
                li { "Public keys;" }

                li { "Anonymized crash reports;" }

                li { "Server logs;" }

                li {
                    "Device identifiers including information about your internet connection, IP address and user agent
                    details;"
                }

                li { "Takedowns and account suspension history." }
            }

            p { "c) Information provided by third-parties:" }

            p {
                "We collect payment invoices provided to us by our third-party payment processors, which includes
                details of your Subscription Plan and any payments made by you in favor of Mango³ in order to receive
                Services from us. We do not collect or store any credit cards or bank information."
            }

            p { "d) Uploaded Information:" }

            p {
                "We use the limited information collected from you only to provide you with our Services."
            }

            p { "e) Information collected from Children:" }

            p {
                "Our Services are not meant for use by children under the age of 13 and do not target children under
                the age of 13. If you are under 13, please do not access or use our services."
            }

            h2 { "4. HOW DO WE USE YOUR INFORMATION?" }

            p { "We use the information that you provide to:" }
            ol {
                li {
                    "Provide our Services that you contract for when you agree to our "

                    LinkToTerms {}

                    " (\"Terms\");"
                }

                li {
                    "Communicate with you in accordance with this Privacy Policy and our "

                    LinkToTerms {}

                    ";"
                }

                li { "Maintain and improve our systems and Services;" }

                li { "Ensure your account's security and mitigate attacks;" }

                li {
                    "Carry out obligations and enforce rights arising from contracts entered into between you and us,
                    including billing and collection;"
                }

                li { "Control access permissions to your Files and your account;" }

                li { "Remove deleted files from users who might have already downloaded them;" }

                li { "Notify you about changes to our Services; and" }

                li { "Anonymize data and aggregate data for statistics." }
            }

            h2 { "5. OUR COOKIE POLICY" }

            p { "We do not set or use Cookies on our website and app." }

            h2 { "6. HOW DO WE PROTECT INFORMATION WE COLLECT?" }

            ol {
                li {
                    "Our Security Measures: We have implemented measures designed to secure your personal information
                    from accidental loss and from unauthorized access, use, alteration, and disclosure. Specifically
                    (a) all information you provide to us is stored on our secure servers behind firewalls, (b) our
                    website and app use an SSL certificate, receive regular security scans, penetration tests and
                    regular malware scans; (c) we require username and passwords for our employees who can access your
                    personal information that we store and/or process, and (d) we actively prevent third parties from
                    getting access to your personal information that we store and/or process."
                }

                li {
                    "Fair Information Practice Principles: In the event of a personal data breach, we will notify you
                    within seventy-two (72) hours via email. We agree to the individual redress principle, according to
                    which you have a right to pursue legally enforceable rights against data collectors and processors
                    who fail to adhere to the law through recourse to courts or a government agency."
                }

                li {
                    p { "Retention of your data:" }

                    p {
                        "a. While you are subscribed: We keep your Files while you are subscribed to our Services,
                        subject to our suspension and termination rights set out in our "

                        LinkToTerms {}

                        "."
                    }

                    p { "b. After suspension/termination of your account:" }

                    ul {
                        li {
                            "Your Files: After account termination, all your Files will be marked for deletion and
                           removed when the next appropriate file purging process is run."
                        }

                        li {
                            "Your Data: After account termination, we may retain your Data for sixty (60) days, or as
                            warranted by your jurisdiction (\"Retention Period\"), unless an enforcement action is
                            likely under our "

                            LinkToTerms {}

                            ". If there is no enforcement action likely or commenced and Retention Period has expired,
                            your Data that identifies you will be anonymized."
                        }
                    }

                    p {
                        "c. Services to other users: If you have shared Data or Files with another Mango³ user, your
                        data might continue to be retained after account termination to allow Services to continue for
                        such other users."
                    }

                    p {
                        "d. Other Instances: We may keep your Files after your account has been suspended or terminated
                        where we consider it necessary for evidential purposes relating to a breach of our "

                        LinkToTerms {}

                        " or with respect to current or anticipated action by any competent enforcement authority or
                        other third party."
                    }
                }

                li {
                    "Your responsibilities: You are responsible for keeping this password and encryption keys
                    confidential. We ask you not to share your password or keys with anyone. Please note that
                    any transmission of personal information is at your own risk and we are not responsible for
                    circumvention of any privacy settings or security measures contained on our Services."
                }
            }

            h2 { "7. DISCLOSURE OF PERSONAL INFORMATION" }

            p {
                "We do not sell, trade, rent, or otherwise transfer personal information to others, unless we provide
                you with advance notice. There are times when Personal Information that you have shared with us may be
                shared by Mango³ with others to enable us to provide you over Services, including contractors, service
                providers, and third parties (\"Partners\") and subsidiaries."
            }

            p {
                "a. Disclosure to provide our Services: We will disclose personal information (i) to fulfill the
                purpose for which you have provided it, and (ii) to enforce or apply our "

                LinkToTerms {}

                " and other agreements, including for billing and collection purposes"
            }

            p {
                "b. Disclosure provided by law or for protection: We will disclose personal information (i) to comply
                with any court order, law, or legal process, including to respond to any government or regulatory
                request, or (ii) if we believe it is necessary or appropriate to protect the rights, property, or
                safety of Mango³, our customers or others."
            }

            p {
                "c. Disclosure in the event of merger or sale: We may disclose personal information in the event of a
                merger, sale of business, etc."
            }

            h2 { "8. BIOMETRIC INFORMATION PRIVACY POLICY" }

            p {
                "We allow you to search and categorize files, including those shared with you, by the faces of the
                people in them. This feature will be enabled on your devices after getting your consent, and can be
                disabled in settings subsequently. This section describes our policies around handling such
                information. Note that sharing a file with a user allows them to extract information like face geometry
                from files on their device, should they wish."
            }

            p {
                "We collect, use, disclose, and safeguard your biometric information when you use Mango³'s platform,
                websites or apps (together \"Services\") in the following manner:"
            }

            ol {
                li {
                    "Biometric Data Defined. As defined in the Biometric Information Privacy Act (\"BIPA\") biometric
                    data includes \"biometric identifiers\" and \"biometric information\" (740 ILCS § 14/1, et seq).
                    \"Biometric identifier\" means a retina or iris scan, fingerprint, voice-print, or scan of a hand
                    and or face geometry. \"Biometric information\" means any information, regardless of how it is
                    captured, converted, stored, or shared, based on an individual's biometric identifier used to
                    identify an individual."
                }

                li {
                    "Biometric Information Collected by Us. Mango³ extracts, stores, uses and discloses Biometric
                    Information like facial geometry from files for the sole purpose of allowing you, or another user
                    with whom you have shared your data with, to search and organize Files based on the people within
                    them. The derivation of such Biometric Information happens only on your device, or on that of
                    another user with whom you have shared your data with, and such Biometric Information is encrypted
                    before such information leaves the device, so we do not have access to any Biometric Information."
                }

                li {
                    "Why We Collect Biometric Information and our Legal basis to Process It. Before processing
                    Biometric Information on their devices, Users will be prompted to affirmatively consent to this
                    Biometric Information Privacy Policy. Biometric Information is processed on their devices only
                    after the User has explicitly provided us consent to do so. Our processing of Biometric
                    Information is for the SOLE PURPOSE of allowing Users to search and organize files on Mango³'s
                    Platform."
                }

                li {
                    "Disclosure of Your Biometric Information. Other than to provide Users with Mango³'s Services,
                    Mango³ will not sell, lease, trade, or otherwise profit from the Biometric Information that it
                    collects. Mango³ will not disclose or disseminate Biometric Information for purposes other than to
                    provide our services to you, unless (a) you or your legally authorized representative provides
                    written consent to such disclosure; or (b) the disclosure is required pursuant to a valid warrant
                    or subpoena issued by a court of competent jurisdiction."
                }

                li {
                    "How We Store Your Biometric Information. Once captured, your Biometric Information is end-to-end
                    encrypted and stored by Mango³ on its servers, in a secure manner. Mango³ shall use a reasonable
                    standard of care to store, transmit, and protect from disclosure any Biometric Information
                    collected. Such storage, transmission, and protection from disclosure shall be performed in a
                    manner that is the same as or more protective than the manner in which Mango³ stores, transmits,
                    and protects from disclosure other confidential and sensitive information, including personal
                    information that can be used to uniquely identify an individual."
                }

                li {
                    "Retention Schedule."

                    p {
                        "a. While you are subscribed: We keep your Biometric Information in an encrypted form while you
                        are subscribed to our Services, subject to the suspension and termination of your Account, in
                        accordance with our "

                        LinkToTerms {}

                        "."
                    }

                    p {
                        "b. After suspension/termination of your account: After account termination, all your Biometric
                        Information will be marked for deletion and removed when the next appropriate purging process
                        is run, but no later than sixty (60) days. Unless otherwise stated in this Section, we may
                        retain your Biometric Information for no more than sixty (60) days after the termination or
                        suspension of your account, unless a longer retention period is required by subpoena, warrant,
                        court order or other legal requirement (\"Retention Period\"). After the Retention Period has
                        expired, we shall permanently destroy such Biometric Information from our systems."
                    }

                    p {
                        "c. Services to other users: If you have shared your Files with another Mango³ user, your
                        Biometric Information might continue to be retained after account termination to allow Services
                        to continue for such other users. After such user's account is terminated or suspended, we
                        shall permanently destroy such Biometric Information from our systems."
                    }
                }
            }

            h2 { "9. FOR OUR EUROPEAN CUSTOMERS AND VISITORS" }

            p { "a. International Data Transfers:" }

            p {
                "We are headquartered in United States of America. Our servers or our third-party hosting services
                partners are located in Europe. Your Personal Information may be accessed by or transferred to us in
                the United States or to our Partners in countries outside of the European Economic Area with a
                different level of data protection than that provided for under European law. As applicable, we require
                that our Partners, affiliates, and subsidiaries sign data processing agreements and/or agree to the
                European Union's model contract for the transfer of personal data to third countries (also known as the
                \"EU Standard Contractual Clauses,\" of June 4, 2021) to ensure adequate protection of your personal
                data. By using our site, you consent to any transfer of your Personal Information out of Europe, UK, or
                Switzerland for processing in the United States of America or other countries. "
            }

            p { "b. Your Rights:" }

            p {
                "We agree to fully comply with the letter and the spirit of the General Data Protection Regulation
                (\"GDPR\"). If you are a resident of or a visitor to Europe, you have certain rights with respect to
                the processing of your Personal Information (referred to as \"Personal Data\" and defined in the GDPR).
                These are:"
            }

            p {
                "i. Access: You can request more information about the Personal Information we hold about you. You can
                also request a copy of the Personal Information."
            }

            p {
                "ii. Rectification: If you believe that any Personal Information, we are holding about you is incorrect
                or incomplete, you can request that we correct or supplement such data."
            }

            p {
                "iii. Objection: You can contact us to let us know that you object to the collection or use of your
                Personal Information for certain purposes."
            }

            p {
                "iv. Erasure: You can request that we erase some or all of your Personal Information from our systems."
            }

            p {
                "v. Restriction of Processing: You can ask us to restrict further processing of your Personal
                Information."
            }

            p {
                "vi. Portability: You have the right to ask for a copy of your Personal Information in a
                machine-readable format. You can also request that we transmit the data to another entity where
                technically feasible."
            }

            p {
                "vii. Withdrawal of Consent: If we are processing your Personal Information based on your consent (as
                indicated at the time of collection of such data), you have the right to withdraw your consent at any
                time. Please note, however, that if you exercise this right, it may limit your ability to use some/ all
                of our Services and you may have to then provide express consent on a case-by-case basis for the use or
                disclosure of certain of your Personal Information, if such use or disclosure is necessary to enable you
                to utilize some or all of our Services."
            }

            p {
                "viii. Right to File Complaint: You have the right to lodge a complaint about our practices with
                respect to your Personal Information with the supervisory authority of your country or EU Member State.
                Please go to "

                a {
                    href: "https://ec.europa.eu/justice/article-29/structure/data-protection-authorities/index_en.htm",
                    rel: "noopener",
                    target: "_blank",
                    "https://ec.europa.eu/justice/article-29/structure/data-protection-authorities/index_en.htm"
                }

                " to locate your Data Protection Authority."
            }

            p { "ix. Response: We will respond to your inquiry within sixty (60) days of the receipt." }

            h2 { "10. FOR OUR CANADIAN USERS" }

            p {
                "This Section supplements the information contained in our Privacy Policy above and applies solely to
                all visitors, users, and others who use or access our Services, who reside in Canada
                (\"consumers\" or \"you\"). We ensure with the Personal Information Protection and Electronics Document
                Act of 2000 (\"PIPEDA\") and any terms defined in the PIPEDA have the same meaning when used in this
                Section."
            }

            p {
                "a. Right to Access Personal Information. You can request to access personal information we hold about
                you. We will first confirm whether you have requested such information, explain how we have used your
                information, provide a list of names with whom your information has been shared and provide a copy of
                your information in an accessible format and make alternative formats available if requested and
                reasonable."
            }

            p {
                "b. Right to Correction/Limited Right to Deletion. You can request us to correct or delete your
                information IF you demonstrate that the personal information, we hold on you is inaccurate. We will
                delete or correct your information within sixty (60) calendar days and will inform the third parties
                with whom we have shared your information."
            }

            p {
                "c. Right to be Forgotten. Your information will be kept with us for as long as it is required for the
                fulfillment of our Services. Unless we otherwise give you notice, we will retain your Information on
                the Mango³ platform on your behalf until such times as you or we terminate your User account."
            }

            p {
                "d. Data Breach Notification. We will send a notification to you as soon as feasible regarding the
                information of any breach that creates a \"real risk of significant harm\" to you. We keep a record of
                every data breach and, on request, provide the Office of the Privacy Commissioner with access to the
                record. "
            }

            h2 { "11. YOUR CALIFORNIA PRIVACY RIGHTS" }

            p {
                "Mango³ does not sell, trade, or otherwise transfer to outside third parties your
                \"Personal Information\" as the term is defined under the California Civil Code Section § 1798.82(h)."
            }

            h2 { "12. CAN-SPAM ACT OF 2003" }

            p {
                "a. Our Responsibility: We comply with the CAN-SPAM Act and its requirements for commercial/promotional
                messages, When we send such a commercial/promotional message we will (a) not use false or misleading
                subjects or email addresses; (b) identify email as an advertisement; (c) include our physical address;
                and (d) monitor our third-party email marketing service provider for compliance, if one is used."
            }

            p {
                "b. Your choice: If you do not wish to receive marketing communications from us, you can use the
                unsubscribe link found at the bottom of the email or email us at "

                a { href: "mailto:support@mango3.app", "support@mango3.app" }

                " to opt out of receiving future marketing emails. You may opt-out/unsubscribe from receiving such
                messages at any time and we will honor your requests. Please note that your opting out from promotional
                messages shall not affect your ability to receive transaction-related emails or non-promotional
                communications regarding us and our Services,"
            }

            h2 { "13. MODIFICATIONS TO OUR PRIVACY POLICY" }

            p {
                "We will update this privacy policy as needed so that it is current, accurate, and as clear as
                possible. Your continued use of our Services confirms your acceptance of our updated Privacy Policy."
            }

            h2 { "14. LIST OF THIRD-PARTY SERVICE PROVIDERS" }

            p {
                "Mango³ uses the following third-party service providers for the provision of services as detailed
                under the "

                LinkToTerms {}

                ", as applicable:"
            }

            ul {
                li {
                    "Polar"

                    ul {
                        li { "Use-case: Payments" }

                        li {
                            "Privacy Policy: "

                            a {
                                href: "https://polar.sh/legal/privacy",
                                rel: "noopener",
                                target: "_blank",
                                "https://polar.sh/legal/privacy"
                            }
                        }
                        li {
                            "Contact: "

                            a { href: "support@polar.sh", "support@polar.sh" }
                        }
                    }
                }

                li {
                    "Cloudflare"

                    ul {
                        li { "Use-case: Networking" }

                        li {
                            "Privacy Policy: "

                            a {
                                href: "https://www.cloudflare.com/privacypolicy/",
                                rel: "noopener",
                                target: "_blank",
                                "https://www.cloudflare.com/privacypolicy/"
                            }
                        }

                        li {
                            "Contact: "

                            a { href: "mailto:privacyquestions@cloudflare.com",
                                "privacyquestions@cloudflare.com"
                            }
                        }
                    }
                }

                li {
                    "InterServer"

                    ul {
                        li { "Use-case: Compute, storage" }

                        li {
                            "Privacy Policy: "

                            a {
                                href: "https://www.interserver.net/privacy-policy.html",
                                rel: "noopener",
                                target: "_blank",
                                "https://www.interserver.net/privacy-policy.html"
                            }
                        }

                        li {
                            "Contact: "

                            a {
                                href: "https://www.interserver.net/contact-information.html",
                                rel: "noopener",
                                target: "_blank",
                                "https://www.interserver.net/contact-information.html"
                            }
                        }
                    }
                }

                li {
                    "GitHub"

                    ul {
                        li { "Use-case: Source code repository, compute, deployment" }

                        li {
                            "Privacy Policy: "

                            a {
                                href: "https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement",
                                rel: "noopener",
                                target: "_blank",
                                "https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement"
                            }
                        }

                        li {
                            "Contact: "

                            a {
                                href: "https://support.github.com/",
                                rel: "noopener",
                                target: "_blank",
                                "https://support.github.com/"
                            }
                        }
                    }
                }

                li {
                    "Mailersend"

                    ul {
                        li { "Use-case: Email communications" }

                        li {
                            "Privacy Policy: "

                            a {
                                href: "https://www.mailersend.com/legal/privacy-policy",
                                rel: "noopener",
                                target: "_blank",
                                "https://www.mailersend.com/legal/privacy-policy"
                            }
                        }

                        li {
                            "Contact: "

                            a {
                                href: "https://www.mailersend.com/contact-us",
                                rel: "noopener",
                                target: "_blank",
                                "https://www.mailersend.com/contact-us"
                            }
                        }
                    }
                }
            }

            h2 { "15. COPYRIGHT INFRINGEMENT" }

            p {
                "If you believe that any content on our website or app, violates your copyright, and you wish to have
                the allegedly infringing material removed, please follow the instructions in our "

                LinkToTerms {}

                " to submit a DMCA Takedown Notice to us."
            }

            h2 { "16. THIRD-PARTY WEBSITES" }

            p {
                "We do allow links to third-party web pages. These third-party sites have separate and independent
                privacy policies. We, therefore, have no responsibility or liability for the content and activities of
                these linked sites."
            }

            h2 { "17. CONTACT US" }

            p {
                "Email us at "

                a { href: "mailto:support@mango3.app", "support@mango3.app" }

                " with any questions about this Privacy Policy or how we process your information. We'll be happy to
                help!"
            }
        }
    }
}
