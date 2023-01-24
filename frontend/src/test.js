import Table from "./character/character";
import { useState, useEffect } from "react";

function App() {
  const [data, setData] = useState([]);

  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ character_name: "Brando" }),
  };

  const fetchData = () => {
    fetch("http://localhost:8000/character", requestOptions)
      .then((response) => response.json())
      .then((actualData) => {
        setData(actualData);
        console.log(data);
      })
      .catch((err) => {
        console.log(err.message);
      });
  };

  // useEffect(() => {
  //   fetchData();
  // }, []);

  const getHeadings = () => {
    console.log(data);
    return Object.keys(data.character);
  };

  return (
    <div className="container">
      <button onClick={fetchData}>Get Character</button>
      {/* <Table theadData={getHeadings()} tbodyData={data[0].character} /> */}
    </div>
  );
}
export default App;

// import Table from "./character/character";
// import data from "./data/character.json";

// function App() {
//   const getCharacter = () => {
//     return data[0].character;
//   };

//   console.log(getCharacter());

//   const getHeadings = () => {
//     return Object.keys(data[0].character);
//   };

//   console.log(getHeadings());

//   return (
//     <div className="container">
//       <Table theadData={getHeadings()} tbodyData={getCharacter()} />
//     </div>
//   );
// }
// export default App;
