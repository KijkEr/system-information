import React, { useState, useEffect } from "react";
import Table from "./Table";

function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    fetch("http://localhost:6969/system_information")
      .then((response) => response.json())
      .then((data) => setData(data))
      .catch((error) => console.error(error));
  }, []);

  if (!data) {
    return <p>Loading...</p>;
  }

  return (
    <div>
      <h1>System Stats</h1>
      <Table data={data} />
    </div>
  );
}

export default App;
