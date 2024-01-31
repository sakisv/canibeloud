module "server" {
  source      = "../modules/server"
  name_prefix = "canibeloud-production"
  server_type = "cax11" # arm64 - 2vCPU, 4GB RAM, 40GB ssd
  location    = "hel1"
  dns_records = [
    { "name" : "@", "use_proxy" : false }, # TODO: change when caddy is configured
    { "name" : "www", "use_proxy" : false },
  ]
  cloudflare_zone_id = module.canibeloud_zone.zone_id
  ssh_key_ids        = module.ssh_keys.key_ids
  environment        = "production"
}

module "canibeloud_zone" {
  source    = "../modules/cloudflare"
  zone_name = "canibeloud.com"
  ssl_mode  = "strict"
}

module "ssh_keys" {
  source = "../modules/hetzner_ssh_keys"
}
