import Table from "./character/character";
import data from "./data/character.json";

function App() {
  const getHeadings = () => {
    return Object.keys(data[0].character[0]);
  };

  console.log(getHeadings());
  console.log(data[0].character);

  return (
    <div className="container">
      <Table theadData={getHeadings()} tbodyData={data[0].character} />
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
