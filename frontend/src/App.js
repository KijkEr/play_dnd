import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [data, setData] = useState([]);

  const fetchData = () => {
    fetch(`http://localhost:8000/character`)
      .then((response) => response.json())
      .then((actualData) => {
        console.log(actualData);
        setData(actualData);
        console.log(data);
      })
      .catch((err) => {
        console.log(err.message);
      });
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <div className="App">
      <tbody>
        <tr>
          <th>Name</th>
          <th>Level</th>
          <th>Race</th>
          <th>Class</th>
          <th>Sub Class</th>
          <th>Proficiency</th>
        </tr>
        <tr>
          <td>{data.character_name}</td>
          <td>{data.level}</td>
          <td>{data.race}</td>
          <td>{data.class}</td>
          <td>{data.sub_class}</td>
          <td>{data.proficiency}</td>
        </tr>
      </tbody>
    </div>
  );
}

export default App;