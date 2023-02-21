use crate::{
    api::ApiResource,
    conn::{Headers, Payload},
    models, opts, Result,
};

use containers_api::url;

impl_api_ty!(
    Network => name
);

impl Network {
    api_doc! {
    Network => DeleteLibpod
    |
    /// Delete this container. To delete this network forcefully use
    /// [`Network::remove`](Network::remove).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.networks().get("some-network").delete().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn delete(&self) -> Result<Vec<models::NetworkRmReport>> {
        self.podman
            .delete_json(&format!("/libpod/networks/{}", &self.name))
            .await
    }}

    api_doc! {
    Network => DeleteLibpod
    |
    /// Force remove this network removing associated containers. To delete network normally use
    /// [`Network::delete`](Network::delete).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.networks().get("some-network").remove().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn remove(&self) -> Result<Vec<models::NetworkRmReport>> {
        let ep = url::construct_ep(
            format!("/libpod/networks/{}", &self.name),
            Some(url::encoded_pair("force", true.to_string())),
        );
        self.podman.delete_json(&ep).await
    }}

    api_doc! {
    Network => ExistsLibpod
    |
    /// Quick way to determine if a network exists by name or id.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.networks().get("some-network").exists().await {
    ///         Ok(exists) => if exists {
    ///             println!("network exists!");
    ///         } else {
    ///             println!("network doesn't exists!");
    ///         },
    ///         Err(e) => eprintln!("check failed: {}", e),
    ///     }
    /// };
    /// ```
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Networks, &self.name)
            .await
    }}

    api_doc! {
    Network => InspectLibpod
    |
    /// Display low level configuration for this CNI network.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.networks().get("some-network").inspect().await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn inspect(&self) -> Result<models::Network> {
        self.podman
            .get_json(&format!("/libpod/networks/{}/json", &self.name))
            .await
    }}

    api_doc! {
    Network => DisconnectLibpod
    |
    /// Disconnect a container from this network.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::NetworkDisconnectOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .networks()
    ///         .get("some-network")
    ///         .disconnect_container(
    ///             &NetworkDisconnectOpts::builder()
    ///                 .container("containerid")
    ///                 .force(true)
    ///                 .build()
    ///         )
    ///         .await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn disconnect_container(&self, opts: &opts::NetworkDisconnectOpts) -> Result<()> {
        self.podman
            .post(
                &format!("/libpod/networks/{}/disconnect", &self.name),
                Payload::Json(opts.serialize_vec()?),
                Headers::none(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! {
    Network => ConnectLibpod
    |
    /// Connect a container to this network.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::NetworkConnectOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .networks()
    ///         .get("some-network")
    ///         .connect_container(
    ///             &NetworkConnectOpts::builder()
    ///                 .container("containerid")
    ///                 .interface_name("eno128")
    ///                 .build()
    ///         )
    ///         .await {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn connect_container(&self, opts: &opts::NetworkConnectOpts) -> Result<()> {
        self.podman
            .post(
                &format!("/libpod/networks/{}/connect", &self.name),
                Payload::Json(opts.serialize_vec()?),
                Headers::none(),
            )
            .await
            .map(|_| ())
    }}
}

impl Networks {
    api_doc! {
    Network => CreateLibpod
    |
    /// Quick way to determine if a network exists by name or id.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     use podman_api::opts::NetworkCreateOpts;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman
    ///         .networks()
    ///         .create(&NetworkCreateOpts::builder().name("test-network").build())
    ///         .await
    ///     {
    ///         Ok(info) => println!("{:?}", info),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn create(&self, opts: &opts::NetworkCreateOpts) -> Result<models::Network> {
        self.podman
            .post_json(
                "/libpod/networks/create",
                Payload::Json(opts.serialize_vec()?),
                Headers::none(),
            )
            .await
    }}

    api_doc! {
    Network => ListLibpod
    |
    /// List network configurations.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.networks().list(&Default::default()).await {
    ///         Ok(networks) => println!("{:?}", networks),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn list(&self, opts: &opts::NetworkListOpts) -> Result<Vec<models::Network>> {
        let ep = url::construct_ep("/libpod/networks/json", opts.serialize());
        self.podman.get_json(&ep).await
    }}

    api_doc! {
    Network => PruneLibpod
    |
    /// Delete unused networks.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// async {
    ///     use podman_api::Podman;
    ///     let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    ///     match podman.networks().prune(&Default::default()).await {
    ///         Ok(report) => println!("{:?}", report),
    ///         Err(e) => eprintln!("{}", e),
    ///     }
    /// };
    /// ```
    pub async fn prune(
        &self,
        opts: &opts::NetworkPruneOpts,
    ) -> Result<Vec<models::NetworkPruneReport>> {
        let ep = url::construct_ep("/libpod/networks/prune", opts.serialize());
        self.podman
            .post_json(&ep, Payload::empty(), Headers::none())
            .await
    }}
}
