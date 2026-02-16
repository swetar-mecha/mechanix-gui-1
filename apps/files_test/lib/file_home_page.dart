import 'dart:io';

import 'package:flutter/material.dart';

class FileHomePage extends StatelessWidget {
  const FileHomePage({super.key});

  @override
  Widget build(BuildContext context) {
    const String homeDir = '/home/mecha';

    return Scaffold(
      appBar: AppBar(title: const Text('Files')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          ListTile(
            leading: const Icon(Icons.home),
            title: const Text('Home directory'),
            onTap: () => _navigateToDirectory(context, homeDir, "Home"),
          ),
          ListTile(
            leading: const Icon(Icons.history),
            title: const Text('Recents'),
            onTap: () => _navigate(context),
          ),
          ListTile(
            leading: const Icon(Icons.download),
            title: const Text('Downloads'),
            onTap: () => _navigate(context),
          ),
          ListTile(
            leading: const Icon(Icons.description),
            title: const Text('Documents'),
            onTap: () => _navigate(context),
          ),
        ],
      ),
    );
  }

  void _navigateToDirectory(BuildContext context, String path, String title) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => HomePageView(directoryPath: path, title: title),
      ),
    );
  }

  void _navigate(BuildContext context) {
    Navigator.push(
      context,
      MaterialPageRoute(builder: (_) => const HardCodedTextPage()),
    );
  }
}

/// ---------------------------------------------------------------------------
/// DIRECTORY LIST PAGE
/// ---------------------------------------------------------------------------

class HomePageView extends StatefulWidget {
  final String directoryPath;
  final String title;

  const HomePageView({
    super.key,
    required this.directoryPath,
    required this.title,
  });

  @override
  State<HomePageView> createState() => _HomePageViewState();
}

class _HomePageViewState extends State<HomePageView> {
  late final Future<List<Directory>> _directoriesFuture;

  @override
  void initState() {
    super.initState();
    _directoriesFuture = _fetchDirectories(widget.directoryPath);
  }

  Future<List<Directory>> _fetchDirectories(String path) async {
    final dir = Directory(path);

    if (!await dir.exists()) {
      return [];
    }

    final entities =
        await dir
            .list(followLinks: false)
            .where((e) => e is Directory)
            .cast<Directory>()
            .toList();

    entities.sort((a, b) => a.path.compareTo(b.path));

    return entities;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(widget.title)),
      body: FutureBuilder<List<Directory>>(
        future: _directoriesFuture,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          }

          if (snapshot.hasError) {
            return Center(
              child: Text(
                'Failed to load directories',
                style: Theme.of(context).textTheme.bodyLarge,
              ),
            );
          }

          final directories = snapshot.data ?? [];

          if (directories.isEmpty) {
            return const Center(child: Text('No directories found'));
          }

          return ListView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: directories.length,
            itemBuilder: (context, index) {
              final dir = directories[index];
              final name = dir.path.split('/').last;

              return ListTile(
                leading: const Icon(Icons.folder),
                title: Text(name),
                onTap: () {
                  // future: navigate deeper
                },
              );
            },
          );
        },
      ),
    );
  }
}

/// ---------------------------------------------------------------------------
/// PLACEHOLDER PAGE
/// ---------------------------------------------------------------------------

class HardCodedTextPage extends StatelessWidget {
  const HardCodedTextPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Details')),
      body: const Center(
        child: Text(
          'This is a hard-coded text widget',
          style: TextStyle(fontSize: 24),
        ),
      ),
    );
  }
}
