#[allow(dead_code)]
mod s;

#[cfg(test)]
mod tests {
    mod security {

        #[test]
        fn test_rcgen() {
            extern crate rcgen;
            use rcgen::generate_simple_self_signed;
            let subject_alt_names =
                vec!["hello.world.happy".to_string(), "localhost".to_string()];

            let cert = generate_simple_self_signed(subject_alt_names).unwrap();
            // The certificate is now valid for localhost and the domain "hello.world.example"
            println!("{}", cert.serialize_pem().unwrap());
            println!("{}", cert.serialize_private_key_pem());
            
            assert!(true);
        }
    }
}
