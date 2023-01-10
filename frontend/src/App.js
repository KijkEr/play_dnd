import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [data, setData] = useState([]);

  const fetchData = () => {
    fetch(`http://localhost:8000/character`)
      .then((response) => response.json())
      .then((actualData) => {
        setData(actualData);
        console.log(data);
        console.log(Object.keys(data[0]));
      })
      .catch((err) => {
        console.log(err.message);
      });
  };

  useEffect(() => {
    fetchData();
  }, []);

  const [roll, setRoll] = useState([]);

  const fetchRoll = () => {
    fetch("http://localhost:8000/roll_dice", { method: "GET" })
      .then((response) => response.json())
      .then((actualRoll) => {
        setRoll(actualRoll);
      });
  };

  return (
    <div className="container">
      <div className="row align-items-start">
        <div className="col-12">
          <div className="top-spacer"></div>
        </div>
      </div>

      <div className="row align-items-center">
        <div className="col-3">
          <div className="spacer"></div>
        </div>

        <div className="col-6">
          <div className="myBackground table-responsive">
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
          <div className="mid-spacer"></div>
          <div className="text-center">
            <button onClick={fetchRoll}>Roll Dice</button>
            <p className="font-weight-bold">Roll: {roll.total}</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
