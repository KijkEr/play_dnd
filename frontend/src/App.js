import { useState } from "react";
import "./App.css";

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

  const [roll, setRoll] = useState([]);

  const fetchRoll = () => {
    fetch("http://localhost:8000/roll_dice", { method: "GET" })
      .then((response) => response.json())
      .then((actualRoll) => {
        setRoll(actualRoll);
      });
  };

  return (
    <div>
      <div>
        <form>
          <label>
            Character Name:
            <input type="text" name="name" />
          </label>
          <input type="submit" name="character" onClick={fetchData} />
        </form>
      </div>
      <div>
        <button onClick={fetchData}>Get Character</button>
      </div>
      <div>
        <table className="table">
          <thead className="thead-dark">
            <tr>
              <th>Name</th>
              <th>Level</th>
              <th>Race</th>
              <th>Class</th>
              <th>Sub Class</th>
              <th>Proficiency</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>{data.character_name}</td>
              <td>{data.level}</td>
              <td>{data.race}</td>
              <td>{data.class}</td>
              <td>{data.sub_class}</td>
              <td>{data.proficiency}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div className="text-center">
        <button onClick={fetchRoll}>Roll Dice</button>
        <p className="font-weight-bold">Roll: {roll.total}</p>
      </div>
    </div>
  );
}

export default App;
