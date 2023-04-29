import React from "react";

function Table(props) {
  const { total_memory, free_memory, cpu_count, cpu_usage } = props.data;

  const cpuUsageRows = cpu_usage.map((usage, index) => (
    <tr key={index}>
      <td>{index + 1}</td>
      <td>{usage.toFixed(2)}%</td>
    </tr>
  ));

  return (
    <table>
      <tbody>
        <tr>
          <td>Total Memory:</td>
          <td>{total_memory}</td>
        </tr>
        <tr>
          <td>Free Memory:</td>
          <td>{free_memory}</td>
        </tr>
        <tr>
          <td>CPU Count:</td>
          <td>{cpu_count}</td>
        </tr>
        <tr>
          <td colSpan={2}>CPU Usage:</td>
        </tr>
        {cpuUsageRows}
      </tbody>
    </table>
  );
}

export default Table;
