use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};

pub fn sniff(intf: String, f: &mut FnMut(&[u8])) {
    let interface_name = intf.clone();
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(interface_names_match)
        .unwrap_or_else(|| panic!("interface {} not found", intf));

    // Create a new channel, dealing with layer 2 packets
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    loop {
        match rx.next() {
            Ok(packet) => f(packet),
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
