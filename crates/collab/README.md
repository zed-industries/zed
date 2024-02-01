# Zed Server

This crate is what we run at https://collab.zed.dev.

It contains our back-end logic for collaboration, to which we connect from the Zed client via a websocket after authenticating via https://zed.dev, which is a separate repo running on Vercel.

# Deployment

We run two instances of collab:

* Staging (https://staging-collab.zed.dev)
* Production (https://collab.zed.dev)

Both of these run on the Kubernetes cluster hosted in Digital Ocean.

Deployment is triggered by pushing to the `collab-staging` (or `collab-production`) tag in Github. The best way to do this is:

* `./script/deploy-collab staging`
* `./script/deploy-collab production`

You can tell what is currrently deployed with `./script/what-is-deployed`.
