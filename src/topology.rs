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

    pub fn get_cpu_port(&mut self, _lala: &str) -> u32 {
        panic!("get_cpu_port");
    }

    /// Return the Thrift port used to communicate with the P4 switch.
    pub fn get_thrift_port(&mut self, switch: &str) -> u32 {
        if self.node(switch)["subtype"] != "p4switch" {
            panic!(format!("{} is not a P4 switch", switch));
        }
        self.node(switch)["thrift_port"].as_u64().unwrap() as u32
    }
}
