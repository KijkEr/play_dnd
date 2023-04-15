import React, { useEffect } from "react";

export default function Table({ character }) {
  const [data, setData] = useState([]);

  const requestOptions = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ character_name: "Brando" }),
  };

  useEffect(() => {
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
    const getHeadings = () => {
      console.log(data);
      let theadData = Object.keys(data.character);
    };
  });

  return (
    <table>
      <thead>
        <tr>
          {theadData.map((heading) => {
            return <th key={heading}>{heading}</th>;
          })}
        </tr>
      </thead>
      <tbody>
        {tbodyData.map((row, index) => {
          return (
            <tr key={index}>
              {theadData.map((key, index) => {
                return <td key={row[key]}>{row[key]}</td>;
              })}
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}
