import 'package:flutter/material.dart';

void main() {
  runApp(
    MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Center(
            child: Text('I Am Rich'),
          ),
          backgroundColor: Color.fromARGB(200, 255, 0, 0),
        ),
        backgroundColor: Colors.blueGrey[900],
        body: Center(
          child: Image(
            image: AssetImage('images/diamond.png'),
            fit: BoxFit.fill,
          ),
        ),
      ),
    ),
  );
}
