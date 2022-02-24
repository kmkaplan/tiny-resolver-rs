use std::net::IpAddr;
use trust_dns_client::{
    rr,
    error::ClientResult,
    op::DnsResponse,
    udp::UdpClientConnection,
    client::{
        Client,
        SyncClient
    }
};

fn resolve(name: &rr::Name) -> ClientResult<IpAddr> {
    // We always start with a root nameserver
    let mut nameserver = "198.41.0.4".parse().unwrap();
    loop {
        let response = dns_query(&name, nameserver)?;
        if let Some(ip) = get_answer(&response) {
	    // Best case: we get an answer to our query and we're done
            return Ok(ip);
        } else if let Some(name) = get_cname(&response) {
            return resolve(&name);
        } else if let Some(ip) = get_glue(&response) {
	    // Second best: we get a "glue record" with the *IP address* of another nameserver to query
            nameserver = ip;
        } else if let Some(ns) = get_ns(&response) {
	    // Third best: we get the *domain name* of another nameserver to query, which we can look up the IP for
            nameserver = resolve(&ns)?;
        } else {
	    // If there's no A record we just panic, this is not a very good
	    // resolver :)
            Err("there's no A record, this is not a very good resolver :)")?
        }
    }
}

fn get_answer(response: &DnsResponse) -> Option<IpAddr> {
    for rr in response.answers() {
        if let rr::RData::A(ip) = rr.rdata() {
            println!("  {rr}");
            return Some((*ip).into());
        }
    }
    None
}

fn get_cname(response: &DnsResponse) -> Option<rr::Name> {
    for rr in response.answers() {
        if let rr::RData::CNAME(name) = rr.rdata() {
            println!("  {rr}");
            return Some(name.clone());
        }
    }
    None
}

fn get_glue(response: &DnsResponse) -> Option<IpAddr> {
    for rr in response.additionals() {
        if let rr::RData::A(ip) = rr.rdata() {
            println!("  {rr}");
            return Some((*ip).into());
        }
    }
    None
}

fn get_ns(response: &DnsResponse) -> Option<&rr::Name> {
    for rr in response.name_servers() {
        if let rr::RData::NS(name) = rr.rdata() {
            println!("  {rr}");
            return Some(name);
        }
    }
    None
}

fn dns_query(name: &rr::Name, server: IpAddr) -> ClientResult<DnsResponse> {
    println!("dig -r @{server} {name}");
    SyncClient::new(
        UdpClientConnection::new((server, 53).into()).unwrap()
    )
        .query(&name, rr::DNSClass::IN, rr::RecordType::A)
}

fn main() -> ClientResult<()> {
    let name = std::env::args().nth(1).unwrap();
    let name: rr::Name = name.parse::<rr::Name>().unwrap();
    println!("Result: {}", resolve(&name)?);
    Ok(())
}
