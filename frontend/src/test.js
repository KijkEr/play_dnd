import Table from "./character/character";
import data from "./data/character.json";

function App() {
  const getHeadings = () => {
    return Object.keys(data[0]);
  };

  return (
    <div className="container">
      <Table theadData={getHeadings()} tbodyData={data} />
    </div>
  );
}
export default App;
