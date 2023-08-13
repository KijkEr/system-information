import React from "react";

function Table(props) {
  const { data } = props;

  const headers = Object.keys(data);
  const rows = headers.map((header, index) => {
    if (header === "cpu_usage") {
      const cpuUsageRows = data[header].map((usage, index) => (
        <tr key={index}>
          <td>{`CPU ${index + 1}`}</td>
          <td>{`${usage.toFixed(2)}%`}</td>
        </tr>
      ));
      return cpuUsageRows;
    } else {
      return (
        <tr key={index}>
          <td>{header}</td>
          <td>{data[header]}</td>
        </tr>
      );
    }
  });

  return (
    <table>
      <tbody>{rows}</tbody>
    </table>
  );
}

export default Table;
