use serde_json::Value;

pub struct Topology {
    network: Value,
}

impl Topology {
    pub fn new(filename: &str) -> Topology {
        let network = Topology::load(filename);
        Topology { network }
    }

    fn load(filename: &str) -> Value {
        let file = std::fs::File::open(filename).expect("can't read topology file");
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).expect("can't parse the topology file")
    }

    /// Be cautionous you get the real value and not a copy
    fn node(&mut self, name: &str) -> &Value {
        &self.network[name]
    }

    pub fn get_cpu_port_intf(&mut self, _lala: &str) -> String {
        panic!("get_cpu_port_intf");
    }

    /// Returns the port index of p4switch's cpu port
    /// Args:
    ///     p4switch: name of the p4 switch
    ///     cpu_node: name of the cpu-node (usually a bridge)
    ///     Returns: index
    pub fn get_cpu_port_index(&mut self, p4switch: &str, cpu_node: Option<&str>) -> u32 {
        let cpu_node = cpu_node.unwrap_or("sw-cpu");
        if self.is_p4switch(p4switch) && self[p4switch].get(cpu_node, None) {
            return self.node(p4switch)[cpu_node].get("intf").unwrap();
        }
        panic!("Switch {} has no cpu port", p4switch);
    }

    /// Return the Thrift port used to communicate with the P4 switch.
    pub fn get_thrift_port(&mut self, switch: &str) -> u32 {
        if self.node(switch)["subtype"] != "p4switch" {
            panic!(format!("{} is not a P4 switch", switch));
        }
        self.node(switch)["thrift_port"].as_u64().unwrap() as u32
    }
}
