const fetchData = async () => {
  const response = await fetch("https://wth3l-tiaaa-aaaap-aa5uq-cai.icp0.io/get_asset_data_with_proof?pair_id=USD");
  const data = await response.json();
  console.log(data);
}

fetchData();