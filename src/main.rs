mod simple_switch;
mod sniff;
mod topology;

struct Controller {
    topo: topology::Topology,
    sw_name: String,
    thrift_port: u32,
    cpu_port: u32,
    controller: simple_switch::SimpleSwitch,
}

impl Controller {
    fn new(sw_name: &str) -> Controller {
        let mut topo = topology::Topology::new("topology.db");
        let thrift_port = topo.get_thrift_port(&sw_name);
        Controller {
            sw_name: sw_name.to_string(),
            thrift_port,
            cpu_port: topo.get_cpu_port_index(&sw_name, None),
            controller: simple_switch::SimpleSwitch::new(thrift_port),
            topo,
        }
    }

    fn init(&mut self) -> &mut Self {
        self.controller.reset_state();
        //        self.add_broadcast_groups();
        //        self.add_mirror();
        self
    }

    /*
    fn add_mirror(&mut self) {
        if self.cpu_port {
            self.controller.mirroring_add(100, cpu_port);
        }
    }

    fn add_broadcast_groups(&mut self) {
        let interface_to_port = self
            .topo
            .get(self.sw_name)
            .unwrap()
            .get("interface_to_port")
            .unwrap()
            .clone();
        interface_to_port.pop("lo", None);
        interface_to_port.pop(self.topo.get_cpu_port_intf(self.sw_name), None);

        let mut mc_grp_id = 1;
        let mut rid = 0;

        for (key, ingress_port) in interface_to_port {
            port_list
        }
    }
    */

    fn recv_msg_cpu(&mut self, _pkt: &[u8]) {
        println!("Got a packet!");
    }

    fn run(&mut self) {
        let cpu_port_intf = self
            .topo
            .get_cpu_port_intf(&self.sw_name)
            .replace("eth0", "eth1");
        sniff::sniff(cpu_port_intf, &mut |p| self.recv_msg_cpu(p));
    }
}

fn main() {
    // get the switch name: sx
    // if there is not we can stop the program
    let sw_name = std::env::args()
        .nth(1)
        .expect("Need the name of the switch");

    println!("Gonna works with interface {}", sw_name);

    Controller::new(&sw_name).init().run();
}
