


// pub struct Client {
//     rust: RustCodeClient<tonic::transport::Channel>,
//     cpp: CppCodeClient<tonic::transport::Channel>,
//     c: CCodeClient<tonic::transport::Channel>,
// }

// impl Client {
//     pub async fn new(adr: String) -> Result<Self, Box<dyn std::error::Error>> {
//         let rust = RustCodeClient::connect(adr.clone()).await?;
//         let cpp = CppCodeClient::connect(adr.clone()).await?;
//         let c = CCodeClient::connect(adr.clone()).await?;

//         Ok(Self { rust, cpp, c })
//     }

//     pub async fn rust(
//         &mut self,
//         code: String,
//         input: String,
//     ) -> Result<CodeResponse, Box<dyn std::error::Error>> {
//         let request = tonic::Request::new(CodeRequest { code, input });
//         let res = self.rust.take(request).await?;

//         let response = res.into_inner();
//         Ok(response)
//     }

//     pub async fn cpp(
//         &mut self,
//         code: String,
//         input: String,
//     ) -> Result<CodeResponse, Box<dyn std::error::Error>> {
//         let request = tonic::Request::new(CodeRequest { code, input });
//         let res = self.cpp.take(request).await?;

//         let response = res.into_inner();
//         Ok(response)
//     }

//     pub async fn c(
//         &mut self,
//         code: String,
//         input: String,
//     ) -> Result<CodeResponse, Box<dyn std::error::Error>> {
//         let request = tonic::Request::new(CodeRequest { code, input });
//         let res = self.c.take(request).await?;

//         let response = res.into_inner();
//         Ok(response)
//     }
// }
