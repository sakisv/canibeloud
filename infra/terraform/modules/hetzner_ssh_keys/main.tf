locals {
  keys = [
    {
      "name" : "sakis",
      "key" : "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDYhqhmRQV/Rocm0PQwYMD6N/fXAj+jwp8Au07YVhud7RVR6zgcDFiHlrgBjvghFpbYzdDgKnTIkIcyUUZd7hQIobFWJ13rmXwWINW4vEMP8eT0hmhoP1qH5NNIA/pJCprPa2Nwj3bwW17scRvN/3tOo2Ynr2Ca7BHYaf/JK22t4KVOMjdNQmnb/SlVPiTGuAp2hautantvylXbdKbKW5dqrh1uJS8hkbj62rfvF2U/hrFAWOOowQ5LMHpYcVAkBqZRtfC/WhWtyAhvHcCBeuKKJV7/ibCVKyQaZluDDE4nY43af+DcFrGFsl/U0x5Ohd1+R1jh1Bh2z8eZzg283dh3",
    },
    {
      "name" : "ci",
      "key" : "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIA8toXfrP9B45SI8BFPUfNU0LS56+rHyxlBo1pzsk0d6",
    }
  ]
}

resource "hcloud_ssh_key" "keys" {
  for_each = {
    for i, key_record in local.keys :
    key_record.name => key_record
  }
  name       = each.value.name
  public_key = each.value.key
}

output "key_ids" {
  value = [for k in hcloud_ssh_key.keys : k.id]
}
