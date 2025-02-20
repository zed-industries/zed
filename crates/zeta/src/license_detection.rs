use regex::Regex;

/// The most common license locations, with US and UK English spelling.
pub const LICENSE_FILES_TO_CHECK: &[&str] = &["LICENSE", "LICENCE", "LICENSE.txt", "LICENCE.txt"];

pub fn is_license_eligible_for_data_collection(license: &str) -> bool {
    // TODO: Include more licenses later (namely, Apache)
    for pattern in [MIT_LICENSE_REGEX, ISC_LICENSE_REGEX, UPL_LICENSE_REGEX] {
        let regex = Regex::new(pattern.trim()).unwrap();
        if regex.is_match(license.trim()) {
            return true;
        }
    }
    false
}

const MIT_LICENSE_REGEX: &str = r#"
^.*MIT License.*

Copyright.*?

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files \(the "Software"\), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software\.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT\. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE\.$
"#;

const ISC_LICENSE_REGEX: &str = r#"
^ISC License

Copyright.*?

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies\.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS\. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE\.$
"#;

const UPL_LICENSE_REGEX: &str = r#"
Copyright.*?

The Universal Permissive License.*?

Subject to the condition set forth below, permission is hereby granted to any person
obtaining a copy of this software, associated documentation and/or data \(collectively
the "Software"\), free of charge and under any and all copyright rights in the
Software, and any and all patent rights owned or freely licensable by each licensor
hereunder covering either \(i\) the unmodified Software as contributed to or provided
by such licensor, or \(ii\) the Larger Works \(as defined below\), to deal in both

\(a\) the Software, and

\(b\) any piece of software and/or hardware listed in the lrgrwrks\.txt file if one is
    included with the Software \(each a "Larger Work" to which the Software is
    contributed by such licensors\),

without restriction, including without limitation the rights to copy, create
derivative works of, display, perform, and distribute the Software and make, use,
sell, offer for sale, import, export, have made, and have sold the Software and the
Larger Work\(s\), and to sublicense the foregoing rights on either these or other
terms\.

This license is subject to the following condition:

The above copyright notice and either this complete permission notice or at a minimum
a reference to the UPL must be included in all copies or substantial portions of the
Software\.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT\. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE\.$
"#;

#[cfg(test)]
mod tests {
    use unindent::unindent;

    use crate::is_license_eligible_for_data_collection;

    #[test]
    fn test_mit_positive_detection() {
        let example_license = unindent(
            r#"
                MIT License

                Copyright (c) 2024 John Doe

                Permission is hereby granted, free of charge, to any person obtaining a copy
                of this software and associated documentation files (the "Software"), to deal
                in the Software without restriction, including without limitation the rights
                to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
                copies of the Software, and to permit persons to whom the Software is
                furnished to do so, subject to the following conditions:

                The above copyright notice and this permission notice shall be included in all
                copies or substantial portions of the Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
                IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
                FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
                AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
                LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
                OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
                SOFTWARE.
            "#
            .trim(),
        );

        assert!(is_license_eligible_for_data_collection(&example_license));

        let example_license = unindent(
            r#"
                The MIT License (MIT)

                Copyright (c) 2019 John Doe

                Permission is hereby granted, free of charge, to any person obtaining a copy
                of this software and associated documentation files (the "Software"), to deal
                in the Software without restriction, including without limitation the rights
                to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
                copies of the Software, and to permit persons to whom the Software is
                furnished to do so, subject to the following conditions:

                The above copyright notice and this permission notice shall be included in all
                copies or substantial portions of the Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
                IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
                FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
                AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
                LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
                OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
                SOFTWARE.
            "#
            .trim(),
        );

        assert!(is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_mit_negative_detection() {
        let example_license = unindent(
            r#"
                MIT License

                Copyright (c) 2024 John Doe

                Permission is hereby granted, free of charge, to any person obtaining a copy
                of this software and associated documentation files (the "Software"), to deal
                in the Software without restriction, including without limitation the rights
                to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
                copies of the Software, and to permit persons to whom the Software is
                furnished to do so, subject to the following conditions:

                The above copyright notice and this permission notice shall be included in all
                copies or substantial portions of the Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
                IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
                FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
                AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
                LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
                OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
                SOFTWARE.

                This project is dual licensed under the MIT License and the Apache License, Version 2.0.
            "#
            .trim(),
        );

        assert!(!is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_isc_positive_detection() {
        let example_license = unindent(
            r#"
                ISC License

                Copyright (c) 2024, John Doe

                Permission to use, copy, modify, and/or distribute this software for any
                purpose with or without fee is hereby granted, provided that the above
                copyright notice and this permission notice appear in all copies.

                THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
                WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
                MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
                ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
                WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
                ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
                OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
            "#
            .trim(),
        );

        assert!(is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_isc_negative_detection() {
        let example_license = unindent(
            r#"
                ISC License

                Copyright (c) 2024, John Doe

                Permission to use, copy, modify, and/or distribute this software for any
                purpose with or without fee is hereby granted, provided that the above
                copyright notice and this permission notice appear in all copies.

                THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
                WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
                MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
                ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
                WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
                ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
                OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

                This project is dual licensed under the ISC License and the MIT License.
            "#
            .trim(),
        );

        assert!(!is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_upl_positive_detection() {
        let example_license = unindent(
            r#"
                Copyright (c) 2025, John Doe

                The Universal Permissive License (UPL), Version 1.0

                Subject to the condition set forth below, permission is hereby granted to any person
                obtaining a copy of this software, associated documentation and/or data (collectively
                the "Software"), free of charge and under any and all copyright rights in the
                Software, and any and all patent rights owned or freely licensable by each licensor
                hereunder covering either (i) the unmodified Software as contributed to or provided
                by such licensor, or (ii) the Larger Works (as defined below), to deal in both

                (a) the Software, and

                (b) any piece of software and/or hardware listed in the lrgrwrks.txt file if one is
                    included with the Software (each a "Larger Work" to which the Software is
                    contributed by such licensors),

                without restriction, including without limitation the rights to copy, create
                derivative works of, display, perform, and distribute the Software and make, use,
                sell, offer for sale, import, export, have made, and have sold the Software and the
                Larger Work(s), and to sublicense the foregoing rights on either these or other
                terms.

                This license is subject to the following condition:

                The above copyright notice and either this complete permission notice or at a minimum
                a reference to the UPL must be included in all copies or substantial portions of the
                Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
                INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
                PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
                HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
                CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
                OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
            "#
            .trim(),
        );

        assert!(is_license_eligible_for_data_collection(&example_license));
    }

    #[test]
    fn test_upl_negative_detection() {
        let example_license = unindent(
            r#"
                UPL License

                Copyright (c) 2024, John Doe

                The Universal Permissive License (UPL), Version 1.0

                Subject to the condition set forth below, permission is hereby granted to any person
                obtaining a copy of this software, associated documentation and/or data (collectively
                the "Software"), free of charge and under any and all copyright rights in the
                Software, and any and all patent rights owned or freely licensable by each licensor
                hereunder covering either (i) the unmodified Software as contributed to or provided
                by such licensor, or (ii) the Larger Works (as defined below), to deal in both

                (a) the Software, and

                (b) any piece of software and/or hardware listed in the lrgrwrks.txt file if one is
                    included with the Software (each a "Larger Work" to which the Software is
                    contributed by such licensors),

                without restriction, including without limitation the rights to copy, create
                derivative works of, display, perform, and distribute the Software and make, use,
                sell, offer for sale, import, export, have made, and have sold the Software and the
                Larger Work(s), and to sublicense the foregoing rights on either these or other
                terms.

                This license is subject to the following condition:

                The above copyright notice and either this complete permission notice or at a minimum
                a reference to the UPL must be included in all copies or substantial portions of the
                Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
                INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
                PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
                HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
                CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
                OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

                This project is dual licensed under the ISC License and the MIT License.
            "#
            .trim(),
        );

        assert!(!is_license_eligible_for_data_collection(&example_license));
    }
}
