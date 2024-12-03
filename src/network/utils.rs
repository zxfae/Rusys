use nix::net::if_::InterfaceFlags;

fn prefixes(pre: &str) -> bool
{
    //Dont show interfaces with this prefixes->
    //More ?
    let pref = ["docker", "veth", "br-", "virbr", "tun", "vmnet", "vboxnet", "wg", "bond", "dummy"];
    !pref.iter().any(|prefix| pre.starts_with(prefix))
}

pub fn is_phy(name: &str) -> bool
{
    if !prefixes(name)
    {
        return false;
    }

    let flags = nix::net::if_::InterfaceFlags::from_bits_truncate(
        nix::ifaddrs::getifaddrs()
        .unwrap()
        .find(|x| x.interface_name == name)
        .map(|x| x.flags.bits())
        .unwrap_or(0)
    );

    !flags.contains(InterfaceFlags::IFF_LOOPBACK) &&
    !flags.contains(InterfaceFlags::IFF_POINTOPOINT)
}
