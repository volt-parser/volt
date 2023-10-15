use {
    crate::*,
    crate::tree::*,
    speculate::speculate,
};

speculate!{
    describe "syntax display" {
        it "generates lines" {
            let tree = tree!(
                node!("root" => [
                    node!("node" => [
                        leaf!(pos!(0, 0, 0), "leaf"),
                    ]),
                    leaf!(pos!(0, 0, 0), "leaf"),
                    error!("error", [
                        leaf!("leaf")
                    ]),
                ])
            );

            let left = tree.fmt(0).iter().map(|v| v.to_string()).collect::<Vec<String>>().join("\n");

            let right = vec![
                "root",
                "  node",
                "    \"leaf\"",
                "  \"leaf\"",
                "  [ERR] error",
                "    \"leaf\"",
            ].iter().map(|v| v.to_string()).collect::<Vec<String>>().join("\n");

            assert_eq!(left, right);
        }
    }
}
