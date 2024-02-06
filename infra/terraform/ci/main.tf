module "canibeloud_pipeline_creds" {
  source        = "../modules/ssm_credentials/"
  common_prefix = "/concourse/canibeloud/"

  secrets = {
    "dockerhub_api_password"  = data.pass_password.dockerhub_api_password.password
    "dockerhub_api_username"  = data.pass_password.dockerhub_api_username.password
    "fastmail_username"       = data.pass_password.fastmail_username.password
    "fastmail_password"       = data.pass_password.fastmail_password.password
    "ssh_private_key"         = data.pass_password.ci_ssh_private_key.full
    "ssh_public_key"          = data.pass_password.ci_ssh_public_key.password
    "email_address_sender"    = data.pass_password.ci_email_address_sender.password
    "email_address_recipient" = data.pass_password.ci_email_address_recipient.password
  }
}
